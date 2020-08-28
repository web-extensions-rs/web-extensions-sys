use js_sys::Object;
use wasm_bindgen::prelude::*;

// TODO
#[rustfmt::skip] // rustfmt removes `async` blocks
#[wasm_bindgen]
extern "C" {
    pub type SidebarAction;

    #[wasm_bindgen(catch, method)]
    pub async fn open(this: &SidebarAction) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch, method, js_name = setPanel)]
    pub async fn set_panel(this: &SidebarAction, details: &Object) -> Result<JsValue, JsValue>;
}
