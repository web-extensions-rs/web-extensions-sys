use js_sys::Function;
use wasm_bindgen::prelude::*;

mod browser_action;
mod contextual_identities;
mod downloads;
mod port;
mod runtime;
mod sessions;
mod sidebar_action;
mod storage;
mod tabs;
mod windows;

pub use browser_action::*;
pub use contextual_identities::*;
pub use downloads::*;
pub use port::*;
pub use runtime::*;
pub use sessions::*;
pub use sidebar_action::*;
pub use storage::*;
pub use tabs::*;
pub use windows::*;

pub mod traits {
    pub use crate::storage::{StorageAreaRead, StorageAreaWrite};
}

#[wasm_bindgen]
extern "C" {
    pub type Browser;

    pub static browser: Browser;

    #[wasm_bindgen(method, getter, js_name = browserAction)]
    pub fn browser_action(this: &Browser) -> BrowserAction;

    #[wasm_bindgen(method, getter, js_name = contextualIdentities)]
    pub fn contextual_identities(this: &Browser) -> ContextualIdentities;

    #[wasm_bindgen(method, getter)]
    pub fn downloads(this: &Browser) -> Downloads;

    #[wasm_bindgen(method, getter)]
    pub fn runtime(this: &Browser) -> Runtime;

    #[wasm_bindgen(method, getter)]
    pub fn sessions(this: &Browser) -> Sessions;

    #[wasm_bindgen(method, getter, js_name = sidebarAction)]
    pub fn sidebar_action(this: &Browser) -> SidebarAction;

    #[wasm_bindgen(method, getter)]
    pub fn storage(this: &Browser) -> Storage;

    #[wasm_bindgen(method, getter)]
    pub fn tabs(this: &Browser) -> Tabs;

    #[wasm_bindgen(method, getter)]
    pub fn windows(this: &Browser) -> Windows;
}

#[wasm_bindgen]
extern "C" {
    pub type EventTarget;

    #[wasm_bindgen(method, js_name = addListener)]
    pub fn add_listener(this: &EventTarget, listener: &Function);

    #[wasm_bindgen(method, js_name = removeListener)]
    pub fn remove_listener(this: &EventTarget, listener: &Function);

    #[wasm_bindgen(method, js_name = hasListener)]
    pub fn has_listener(this: &EventTarget, listener: &Function) -> bool;
}
