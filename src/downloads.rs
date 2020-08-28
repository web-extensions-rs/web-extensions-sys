use js_sys::Object;
use wasm_bindgen::prelude::*;

// TODO other methods
#[rustfmt::skip] // rustfmt removes `async` blocks
#[wasm_bindgen]
extern "C" {
    pub type Downloads;

    #[wasm_bindgen(catch, method)]
    pub async fn download(this: &Downloads, info: &Object) -> Result<JsValue, JsValue>;
}
