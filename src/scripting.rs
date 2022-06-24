//! <https://developer.chrome.com/docs/extensions/reference/scripting>

use js_sys::Object;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    pub type Scripting;

    // https://developer.chrome.com/docs/extensions/reference/scripting/#method-insertCSS
    #[wasm_bindgen(method, catch, js_name = insertCSS)]
    pub async fn insert_css(this: &Scripting, options: Object) -> Result<(), JsValue>;

    // https://developer.chrome.com/docs/extensions/reference/scripting/#method-executeScript
    #[wasm_bindgen(method, catch, js_name = executeScript)]
    pub async fn execute_script(this: &Scripting, options: Object) -> Result<JsValue, JsValue>;
}
