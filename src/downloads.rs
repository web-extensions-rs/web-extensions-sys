use js_sys::Object;
use wasm_bindgen::prelude::*;

// TODO other methods
#[wasm_bindgen]
extern "C" {
    pub type Downloads;

    #[wasm_bindgen(catch, method)]
    pub async fn download(this: &Downloads, info: &Object) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch, method)]
    pub async fn search(this: &Downloads, query: &JsValue) -> Result<JsValue, JsValue>;
}
