use js_sys::{Object, Promise};
use wasm_bindgen::prelude::*;

// TODO
#[wasm_bindgen]
extern "C" {
    pub type SidebarAction;

    #[wasm_bindgen(method)]
    pub fn open(this: &SidebarAction) -> Promise;

    #[wasm_bindgen(method, js_name = setPanel)]
    pub fn set_panel(this: &SidebarAction, details: &Object) -> Promise;
}
