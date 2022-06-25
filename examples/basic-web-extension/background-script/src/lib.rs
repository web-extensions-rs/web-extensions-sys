use gloo_console as console;
use js_sys::{Function, Object};
use messages::{Request, Response};
use serde::Serialize;
use wasm_bindgen::{prelude::*, JsCast};
use web_extensions_sys::{chrome, Port, Tab, TabChangeInfo};

const VERSION: &str = env!("CARGO_PKG_VERSION");

type TabId = i32;

#[wasm_bindgen]
pub fn start() {
    console::info!("Start background script");

    let on_message = |message: JsValue, sender, send_response: Function| {
        console::debug!("Received message", &message, &sender);
        if let Some(response) = handle_message(message) {
            let this = JsValue::null();
            send_response.call1(&this, &response).unwrap();
        }
    };
    let closure: Closure<dyn Fn(JsValue, JsValue, Function)> = Closure::new(on_message);
    let callback = closure.as_ref().unchecked_ref();
    chrome.runtime().on_message().add_listener(callback);
    closure.forget();

    let on_tab_changed = |tab_id, change_info: TabChangeInfo, tab: Tab| {
        console::info!("Tab changed", tab_id, &change_info, &tab);
        if change_info.status() == Some("complete".to_string()) {
            if let Some(url) = tab.url() {
                if url.starts_with("http") {
                    console::info!("inject foreground script");
                    wasm_bindgen_futures::spawn_local(inject_frontend(tab_id));
                }
            }
        }
    };
    let listener: Closure<dyn Fn(TabId, TabChangeInfo, Tab)> = Closure::new(on_tab_changed);
    chrome
        .tabs()
        .on_updated()
        .add_listener(listener.as_ref().unchecked_ref());
    listener.forget();

    let on_connect = |port: Port| {
        console::info!("A new port has connected", &port);
        let port_clone = port.clone();
        let on_message = move |msg| {
            console::info!("Received message", &msg);
            if let Some(response) = handle_message(msg) {
                port_clone.post_message(&response);
            }
        };
        let closure: Closure<dyn Fn(JsValue)> = Closure::new(on_message);
        port.on_message()
            .add_listener(closure.as_ref().unchecked_ref());
        closure.forget();
    };
    let closure: Closure<dyn Fn(Port)> = Closure::new(on_connect);
    let callback = closure.as_ref().unchecked_ref();
    chrome.runtime().on_connect().add_listener(callback);
    closure.forget();
}

fn handle_message(msg: JsValue) -> Option<JsValue> {
    msg.into_serde().ok().map(|request| {
        let response = match request {
            Request::Ping => Response::Pong,
            Request::GetOptionsInfo => Response::OptionsInfo {
                version: VERSION.to_string(),
            },
        };
        JsValue::from_serde(&response).unwrap()
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
