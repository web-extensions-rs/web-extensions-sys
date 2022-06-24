use gloo_console as console;
use wasm_bindgen::{prelude::*, JsCast};
use web_extensions_sys::chrome;

#[wasm_bindgen]
pub fn start() {
    console::info!("Start background script");
    let listener: Closure<dyn Fn(JsValue, JsValue, JsValue)> =
        Closure::new(|tab_id, change_info, tab| {
            console::info!("Tab changed", tab_id, change_info, tab);
        });
    chrome
        .tabs()
        .on_updated()
        .add_listener(listener.as_ref().unchecked_ref());
    listener.forget();
}
