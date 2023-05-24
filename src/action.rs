use crate::EventTarget;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    // https://developer.chrome.com/docs/extensions/reference/action/
    pub type Action;

    // https://developer.chrome.com/docs/extensions/reference/action/#event-onClicked
    #[wasm_bindgen(method, getter, js_name = onClicked)]
    pub fn on_clicked(this: &Action) -> EventTarget;

    // https://developer.chrome.com/docs/extensions/reference/action/#method-openPopup
    #[wasm_bindgen(catch, method, js_name = openPopup)]
    pub async fn open_popup(this: &Action, options: &JsValue) -> Result<JsValue, JsValue>;
}
