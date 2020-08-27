use js_sys::{Object, Promise};
use wasm_bindgen::prelude::*;

// TODO other methods
#[wasm_bindgen]
extern "C" {
    pub type Downloads;

    #[wasm_bindgen(method)]
    pub fn download(this: &Downloads, info: &Object) -> Promise;
}
