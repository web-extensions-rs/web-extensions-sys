use crate::Event;
use js_sys::Object;
use wasm_bindgen::prelude::*;

#[rustfmt::skip] // rustfmt removes `async` blocks
#[wasm_bindgen]
extern "C" {
    pub type ContextualIdentities;

    #[wasm_bindgen(catch, method)]
    pub async fn create(this: &ContextualIdentities, details: &Object) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch, method)]
    pub async fn get(this: &ContextualIdentities, cookie_store_id: &str) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch, method)]
    pub async fn query(this: &ContextualIdentities, details: &Object) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch, method)]
    pub async fn update(this: &ContextualIdentities, cookie_store_id: &str, details: &Object) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch, method)]
    pub async fn remove(this: &ContextualIdentities, cookie_store_id: &str) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(method, getter, js_name = onCreated)]
    pub fn on_created(this: &ContextualIdentities) -> Event;

    #[wasm_bindgen(method, getter, js_name = onRemoved)]
    pub fn on_removed(this: &ContextualIdentities) -> Event;

    #[wasm_bindgen(method, getter, js_name = onUpdated)]
    pub fn on_updated(this: &ContextualIdentities) -> Event;
}
