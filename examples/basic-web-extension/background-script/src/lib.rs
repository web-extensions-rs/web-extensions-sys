use gloo_console as console;
use js_sys::Object;
use serde::Serialize;
use wasm_bindgen::{prelude::*, JsCast};
use web_extensions_sys::{chrome, Tab, TabChangeInfo};

type TabId = i32;

#[wasm_bindgen]
pub fn start() {
    console::info!("Start background script");
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
