use gloo_console as console;
use gloo_utils::document;
use messages::{AppRequestPayload, AppResponsePayload, Request, Response};
use wasm_bindgen::prelude::*;
use web_extensions_sys::chrome;

#[wasm_bindgen]
pub fn start() {
    console::info!("Start options script");

    let payload = AppRequestPayload::GetOptionsInfo;
    let msg = JsValue::from_serde(&Request::new(payload)).unwrap();

    wasm_bindgen_futures::spawn_local(async move {
        match chrome.runtime().send_message(None, &msg, None).await {
            Ok(js_value) => {
                if js_value.is_object() {
                    handle_response(js_value);
                } else {
                    console::debug!("The sender has unexpectedly not sent a reply");
                }
            }
            Err(err) => {
                console::error!("Unable to send request", err);
            }
        };
    });
}

fn handle_response(response: JsValue) {
    if let Ok(Response {
        header: _,
        payload: AppResponsePayload::OptionsInfo { version },
    }) = response.into_serde()
    {
        let container = document().query_selector("#version").unwrap().unwrap();
        container.set_inner_html(&format!("Version: {version}"));
    } else {
        console::warn!("Received unexpected message");
    }
}
