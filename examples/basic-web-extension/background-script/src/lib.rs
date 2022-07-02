use gloo_console as console;
use js_sys::{Function, Object};
use messages::{PortRequest, PortResponse, Request, Response, StreamResponse};
use serde::Serialize;
use std::{cell::RefCell, collections::HashMap, sync::Arc};
use wasm_bindgen::{prelude::*, JsCast};
use web_extensions_sys::{chrome, Port, Tab, TabChangeInfo};

const VERSION: &str = env!("CARGO_PKG_VERSION");

type TabId = i32;

#[derive(Default)]
struct App {
    port_counter: RefCell<usize>,
    ports: RefCell<HashMap<usize, Port>>,
}

impl App {
    fn add_port(&self, port: Port) -> usize {
        *self.port_counter.borrow_mut() += 1;
        let port_id = *self.port_counter.borrow();
        self.ports.borrow_mut().insert(port_id, port.clone());
        port_id
    }
    fn send_response_to_port(&self, port_id: usize, response: JsValue) {
        if let Some(port) = self.ports.borrow().get(&port_id) {
            console::debug!("Posting response message on port", port, &response);
            port.post_message(&response);
        } else {
            console::warn!("Expected port ", port_id, "does not exist");
        }
    }
}

#[wasm_bindgen]
pub fn start() {
    console::info!("Starting background script");

    let app = Arc::new(App::default());

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

    let closure: Closure<dyn Fn(Port)> = Closure::new(move |port| {
        on_connect_port(port, &app);
    });
    chrome
        .runtime()
        .on_connect()
        .add_listener(closure.as_ref().unchecked_ref());
    closure.forget();
}

fn on_message(request: JsValue, sender: JsValue, send_response: Function) {
    console::debug!("Received request message", &request, &sender);
    if let Some(response) = handle_request(request) {
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

fn on_connect_port(port: Port, app: &Arc<App>) {
    // TODO: How to notice a disconnection?
    console::info!("Connecting new port", &port);
    let port_id = app.add_port(port.clone());
    let app = Arc::clone(app);
    let on_message = move |request| {
        on_port_message(port_id, &app, request);
    };
    let closure: Closure<dyn Fn(JsValue)> = Closure::new(on_message);
    port.on_message()
        .add_listener(closure.as_ref().unchecked_ref());
    closure.forget();
}

fn on_port_message(port_id: usize, app: &Arc<App>, request: JsValue) {
    console::debug!("Received request message on port", port_id, &request);
    if let Some(response) = handle_port_request(request) {
        app.send_response_to_port(port_id, response);
    }
}

fn handle_request(request: JsValue) -> Option<JsValue> {
    let request = request
        .into_serde()
        .map_err(|err| {
            console::error!("Failed to deserialize request message", &err.to_string());
        })
        .ok()?;
    let response = handle_request_domain(request);
    JsValue::from_serde(&response)
        .map_err(|err| {
            console::error!("Failed to serialize response message", &err.to_string());
        })
        .ok()
}

fn handle_port_request(request: JsValue) -> Option<JsValue> {
    let request = request
        .into_serde()
        .map_err(|err| {
            console::error!(
                "Failed to deserialize port request message",
                &err.to_string()
            );
        })
        .ok()?;
    let response = handle_port_request_domain(request);
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
fn handle_request_domain(request: Request) -> Option<Response> {
    match request {
        Request::GetOptionsInfo => Response::OptionsInfo {
            version: VERSION.to_string(),
        }
        .into(),
    }
}

/// Handle a port-local request.
///
/// Optionally returns a single response.
///
/// TODO: Extract into domain crate
fn handle_port_request_domain(request: PortRequest) -> Option<PortResponse> {
    match request {
        PortRequest::Ping => PortResponse::Pong.into(),
        PortRequest::StreamingExample => {
            PortResponse::StreamingExample(StreamResponse::Accepted).into()
        }
    }
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
