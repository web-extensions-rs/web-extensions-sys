use crate::EventTarget;
use js_sys::Object;
use wasm_bindgen::prelude::*;

#[rustfmt::skip] // rustfmt removes `async` blocks
#[wasm_bindgen]
extern "C" {
    pub type StorageAreaRead;

    #[wasm_bindgen(catch, method, js_name = "getBytesInUse")]
    pub async fn get_bytes_in_use(this: &StorageAreaRead, keys: &JsValue) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch, method)]
    pub async fn get(this: &StorageAreaRead, keys: &JsValue) -> Result<JsValue, JsValue>;
}

#[rustfmt::skip] // rustfmt removes `async` blocks
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = StorageAreaRead)]
    pub type StorageArea;

    #[wasm_bindgen(catch, method)]
    pub async fn set(this: &StorageArea, keys: &Object) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch, method)]
    pub async fn remove(this: &StorageArea, keys: &JsValue) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch, method)]
    pub async fn clear(this: &StorageArea) -> Result<JsValue, JsValue>;
}

#[wasm_bindgen]
extern "C" {
    pub type Storage;

    #[wasm_bindgen(method, getter)]
    pub fn sync(this: &Storage) -> StorageArea;

    #[wasm_bindgen(method, getter)]
    pub fn local(this: &Storage) -> StorageArea;

    #[wasm_bindgen(method, getter)]
    pub fn session(this: &Storage) -> StorageArea;

    #[wasm_bindgen(method, getter)]
    pub fn managed(this: &Storage) -> StorageAreaRead;

    #[wasm_bindgen(method, getter, js_name = onChanged)]
    pub fn on_changed(this: &Storage) -> EventTarget;
}

#[wasm_bindgen]
extern "C" {
    pub type StorageChange;

    #[wasm_bindgen(method, getter, js_name = oldValue)]
    pub fn old_value(this: &StorageChange) -> JsValue;

    #[wasm_bindgen(method, getter, js_name = newValue)]
    pub fn new_value(this: &StorageChange) -> JsValue;
}
