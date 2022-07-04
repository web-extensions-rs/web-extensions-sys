use std::{cell::RefCell, collections::HashMap, rc::Rc};

use gloo_console as console;
use gloo_timers::future::TimeoutFuture;
use js_sys::{Function, Object};
use messages::{
    next_request_id, AppRequest, AppRequestPayload, AppResponse, AppResponsePayload, PortRequest,
    PortRequestPayload, PortResponse, PortResponsePayload, Request, RequestId, Response,
    StreamingFinishedStatus, StreamingResponsePayload, StreamingStartedStatus, INITIAL_REQUEST_ID,
};
use serde::Serialize;
use thiserror::Error;
use wasm_bindgen::{prelude::*, JsCast};

use web_extensions_sys::{chrome, Port, Tab, TabChangeInfo};

const VERSION: &str = env!("CARGO_PKG_VERSION");

type TabId = i32;

type PortId = usize;

const FIRST_PORT_ID: RequestId = 1;

#[derive(Debug)]
struct PortContext {
    port: Port,
    last_request_id: RequestId,
}

impl PortContext {
    const fn new(port: Port) -> Self {
        Self {
            port,
            last_request_id: INITIAL_REQUEST_ID,
        }
    }

    fn next_request_id(&mut self) -> RequestId {
        let next_request_id = next_request_id(self.last_request_id);
        self.last_request_id = next_request_id;
        next_request_id
    }
}

#[derive(Default)]
struct ConnectedPorts {
    last_id: PortId,
    ctx_by_id: HashMap<PortId, PortContext>,
}

#[derive(Debug, Error)]
enum PortError {
    #[error("not connected")]
    NotConnected,
}

impl ConnectedPorts {
    fn connect(&mut self, port: Port) -> Option<PortId> {
        let id = self.last_id.checked_add(1)?;
        debug_assert!(id >= FIRST_PORT_ID);
        let ctx = PortContext::new(port);
        self.ctx_by_id.insert(id, ctx);
        Some(id)
    }

    fn disconnect(&mut self, id: PortId) -> Option<Port> {
        self.ctx_by_id
            .remove(&id)
            .map(|PortContext { port, .. }| port)
    }

    fn post_message_js(&self, id: PortId, msg: &JsValue) -> Result<(), PortError> {
        self.ctx_by_id
            .get(&id)
            .ok_or(PortError::NotConnected)
            .map(|ctx| {
                let PortContext {
                    port,
                    last_request_id: _,
                } = ctx;
                console::debug!("Posting message on port", port, msg);
                port.post_message(msg);
            })
    }

    fn post_message<T: Serialize>(&self, id: PortId, msg: &T) -> Result<(), PortError> {
        self.ctx_by_id
            .get(&id)
            .ok_or(PortError::NotConnected)
            .map(|ctx| {
                let PortContext {
                    port,
                    last_request_id: _,
                } = ctx;
                let msg = match JsValue::from_serde(msg) {
                    Ok(msg) => msg,
                    Err(err) => {
                        console::error!("Failed to serialize message", err.to_string());
                        return;
                    }
                };
                console::debug!("Posting message on port", port, &msg);
                port.post_message(&msg);
            })
    }

    fn next_request_id(&mut self, id: PortId) -> Result<RequestId, PortError> {
        self.ctx_by_id
            .get_mut(&id)
            .ok_or(PortError::NotConnected)
            .map(|ctx| ctx.next_request_id())
    }
}

#[derive(Default)]
struct App {
    last_request_id: RequestId,
    connected_ports: ConnectedPorts,
}

impl App {
    fn next_request_id(&mut self) -> RequestId {
        let next_request_id = next_request_id(self.last_request_id);
        self.last_request_id = next_request_id;
        next_request_id
    }

    fn connect_port(&mut self, port: Port) -> Option<PortId> {
        self.connected_ports.connect(port)
    }

    fn disconnect_port(&mut self, port_id: PortId) -> Option<Port> {
        self.connected_ports.disconnect(port_id)
    }

    fn next_port_request_id(&mut self, port_id: PortId) -> Result<RequestId, PortError> {
        self.connected_ports.next_request_id(port_id)
    }

    fn post_port_message<T: Serialize>(&self, port_id: PortId, msg: &T) -> Result<(), PortError> {
        self.connected_ports.post_message(port_id, msg)
    }

    fn post_port_message_js(&self, port_id: PortId, msg: &JsValue) -> Result<(), PortError> {
        self.connected_ports.post_message_js(port_id, msg)
    }
}

#[wasm_bindgen]
pub fn start() {
    console::info!("Starting background script");

    let app = Rc::new(RefCell::new(App::default()));

    let on_message = {
        let app = Rc::clone(&app);
        move |request, sender, send_response| on_message(&app, request, sender, send_response)
    };
    let closure: Closure<dyn Fn(JsValue, JsValue, Function)> = Closure::new(on_message);
    chrome
        .runtime()
        .on_message()
        .add_listener(closure.as_ref().unchecked_ref());
    closure.forget();

    let closure: Closure<dyn Fn(TabId, TabChangeInfo, Tab)> = Closure::new(on_tab_changed);
    chrome
        .tabs()
        .on_updated()
        .add_listener(closure.as_ref().unchecked_ref());
    closure.forget();

    let on_connect = move |port| {
        on_connect_port(&app, port);
    };
    let closure: Closure<dyn Fn(Port)> = Closure::new(on_connect);
    chrome
        .runtime()
        .on_connect()
        .add_listener(closure.as_ref().unchecked_ref());
    closure.forget();
}

fn on_message(app: &Rc<RefCell<App>>, request: JsValue, sender: JsValue, send_response: Function) {
    console::debug!("Received request message", &request, &sender);
    let request_id = app.borrow_mut().next_request_id();
    if let Some(response) = on_request(app, request_id, request) {
        let this = JsValue::null();
        if let Err(err) = send_response.call1(&this, &response) {
            console::error!(
                "Failed to send response message",
                send_response,
                response,
                err
            );
        }
    }
}

fn on_tab_changed(tab_id: i32, change_info: TabChangeInfo, tab: Tab) {
    console::info!("Tab changed", tab_id, &tab, &change_info);
    if change_info.status().as_deref() == Some("complete") {
        if let Some(url) = tab.url() {
            if url.starts_with("http") {
                console::info!("Injecting foreground script on tab", tab_id, &tab);
                wasm_bindgen_futures::spawn_local(inject_frontend(tab_id));
            }
        }
    }
}

fn on_connect_port(app: &Rc<RefCell<App>>, port: Port) {
    // FIXME: How to notice a disconnection?
    console::info!("Connecting new port", &port);
    let port_id = if let Some(port_id) = app.borrow_mut().connect_port(port.clone()) {
        port_id
    } else {
        console::error!("Failed to connect new port", &port);
        return;
    };
    let on_message = {
        let app = Rc::clone(app);
        move |request| {
            on_port_message(&app, port_id, request);
        }
    };
    let closure: Closure<dyn Fn(JsValue)> = Closure::new(on_message);
    port.on_message()
        .add_listener(closure.as_ref().unchecked_ref());
    closure.forget();
}

fn on_port_message(app: &Rc<RefCell<App>>, port_id: PortId, request: JsValue) {
    console::debug!("Received request message on port", port_id, &request);
    let request_id = match app.borrow_mut().next_port_request_id(port_id) {
        Ok(request_id) => request_id,
        Err(err) => {
            console::warn!(
                "Failed to handle port request",
                port_id,
                request,
                err.to_string()
            );
            return;
        }
    };
    if let Some(response) = on_port_request(app, port_id, request_id, request) {
        if let Err(err) = app.borrow().post_port_message_js(port_id, &response) {
            console::warn!(
                "Failed to post response message to port",
                port_id,
                response,
                err.to_string()
            );
        }
    }
}

fn on_request(app: &Rc<RefCell<App>>, request_id: RequestId, request: JsValue) -> Option<JsValue> {
    let request = request
        .into_serde()
        .map_err(|err| {
            console::error!("Failed to deserialize request message", &err.to_string());
        })
        .ok()?;
    let response = handle_app_request(app, request_id, request);
    JsValue::from_serde(&response)
        .map_err(|err| {
            console::error!("Failed to serialize response message", &err.to_string());
        })
        .ok()
}

fn on_port_request(
    app: &Rc<RefCell<App>>,
    port_id: PortId,
    request_id: RequestId,
    request: JsValue,
) -> Option<JsValue> {
    let request = request
        .into_serde()
        .map_err(|err| {
            console::error!(
                "Failed to deserialize port request message",
                &err.to_string()
            );
        })
        .ok()?;
    let response = handle_port_request(app, port_id, request_id, request);
    JsValue::from_serde(&response)
        .map_err(|err| {
            console::error!(
                "Failed to serialize port response message",
                &err.to_string()
            );
        })
        .ok()
}

/// Handle a (global) request.
///
/// Optionally returns a single response.
///
/// TODO: Extract into domain crate
fn handle_app_request(
    _app: &Rc<RefCell<App>>,
    request_id: RequestId,
    request: AppRequest,
) -> Option<AppResponse> {
    let Request { header, payload } = request;
    let payload: Option<_> = match payload {
        AppRequestPayload::GetOptionsInfo => AppResponsePayload::OptionsInfo {
            version: VERSION.to_string(),
        }
        .into(),
    };
    payload.map(|payload| Response {
        header: header.into_response(request_id),
        payload,
    })
}

/// Handle a port-local request.
///
/// Optionally returns a single response.
///
/// TODO: Extract into domain crate
fn handle_port_request(
    app: &Rc<RefCell<App>>,
    port_id: PortId,
    request_id: RequestId,
    request: PortRequest,
) -> Option<PortResponse> {
    let Request { header, payload } = request;
    let payload: Option<_> = match payload {
        PortRequestPayload::Ping => PortResponsePayload::Pong.into(),
        PortRequestPayload::StartStreaming { num_items } => {
            let status = match num_items {
                0 => StreamingStartedStatus::Rejected {
                    reason: "no items requested".to_string().into(),
                },
                10.. => StreamingStartedStatus::Rejected {
                    reason: "too many items requested".to_string().into(),
                },
                _ => {
                    wasm_bindgen_futures::spawn_local({
                        let header = header.clone();
                        let app = Rc::clone(app);
                        // TODO: Wrap async the task into a control struct that allows aborting it.
                        // All pending tasks need be managed depending on the scope, i.e. app or port.
                        async move {
                            console::debug!("Start streaming");
                            let mut last_item_index = None;
                            for item_index in 0..num_items {
                                let payload = PortResponsePayload::Streaming(
                                    StreamingResponsePayload::Item { index: item_index },
                                );
                                let response = Response {
                                    header: header.clone().into_response(request_id),
                                    payload,
                                };
                                console::debug!(
                                    "Next streaming item response",
                                    format!("{response:?}")
                                );
                                app.borrow().post_port_message(port_id, &response).ok();
                                last_item_index = Some(item_index);
                                // Delay the next (or final) response. Without yielding at some point
                                // the locally spawned task would finish before the started response
                                // could be posted.
                                TimeoutFuture::new(5_000).await;
                            }
                            console::debug!("Finish streaming");
                            let status = if last_item_index.unwrap_or(num_items) < num_items {
                                StreamingFinishedStatus::Completed
                            } else {
                                StreamingFinishedStatus::Aborted { reason: None }
                            };
                            let payload = PortResponsePayload::Streaming(
                                StreamingResponsePayload::Finished {
                                    status,
                                    last_item_index,
                                },
                            );
                            let response = Response {
                                header: header.into_response(request_id),
                                payload,
                            };
                            app.borrow().post_port_message(port_id, &response).ok();
                        }
                    });
                    StreamingStartedStatus::Accepted
                }
            };
            PortResponsePayload::Streaming(StreamingResponsePayload::Started { status }).into()
        }
    };
    // The started response might be posted after the first stream item response
    // or even after the finished response that are all generated asynchronously!
    payload.map(|payload| Response {
        header: header.into_response(request_id),
        payload,
    })
}

// https://developer.chrome.com/docs/extensions/reference/scripting/#type-CSSInjection
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct CssInjection<'a> {
    target: InjectionTarget<'a>,
    #[serde(skip_serializing_if = "Option::is_none")]
    css: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    files: Option<&'a [&'a str]>,
}

// https://developer.chrome.com/docs/extensions/reference/scripting/#type-ScriptInjection
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct ScriptInjection<'a> {
    target: InjectionTarget<'a>,
    #[serde(skip_serializing_if = "Option::is_none")]
    files: Option<&'a [&'a str]>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct InjectionTarget<'a> {
    tab_id: TabId,
    #[serde(skip_serializing_if = "Option::is_none")]
    all_frames: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    frame_ids: Option<&'a [i32]>,
}

async fn inject_frontend(tab_id: TabId) {
    let css_injection = JsValue::from_serde(&CssInjection {
        files: Some(&["foreground-script/style.css"]),
        css: None,
        target: InjectionTarget {
            tab_id,
            all_frames: None,
            frame_ids: None,
        },
    })
    .unwrap();
    console::info!("Inject CSS", &css_injection);
    if let Err(err) = chrome
        .scripting()
        .insert_css(Object::from(css_injection))
        .await
    {
        console::info!("Unable to inject CSS", err);
    }
    let script_injection = JsValue::from_serde(&ScriptInjection {
        files: Some(&[
            "foreground-script/pkg/foreground_script.js",
            "foreground-script/index.js",
        ]),
        target: InjectionTarget {
            tab_id,
            all_frames: None,
            frame_ids: None,
        },
    })
    .unwrap();

    if let Err(err) = chrome
        .scripting()
        .execute_script(Object::from(script_injection))
        .await
    {
        console::info!("Unable to inject JS", err);
    }
}
