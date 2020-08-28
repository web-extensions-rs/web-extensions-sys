use crate::tabs::Tab;
use crate::windows::Window;
use crate::Event;
use js_sys::Object;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    pub type Session;

    #[wasm_bindgen(method, getter, js_name = lastModified)]
    pub fn last_modified(this: &Session) -> f64;

    #[wasm_bindgen(method, getter)]
    pub fn tab(this: &Session) -> Option<Tab>;

    #[wasm_bindgen(method, getter)]
    pub fn window(this: &Session) -> Option<Window>;
}

#[wasm_bindgen]
extern "C" {
    pub type Sessions;

    #[wasm_bindgen(method, getter, js_name = MAX_SESSION_RESULTS)]
    // TODO is u32 correct ?
    pub fn max_session_results(this: &Sessions) -> u32;

    #[wasm_bindgen(catch, method, js_name = forgetClosedTab)]
    pub async fn forget_closed_tab(this: &Sessions, window_id: i32, session_id: &str) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch, method, js_name = forgetClosedWindow)]
    pub async fn forget_closed_window(this: &Sessions, session_id: &str) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch, method, js_name = getRecentlyClosed)]
    pub async fn get_recently_closed(this: &Sessions, filter: Option<&Object>) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch, method)]
    pub async fn restore(this: &Sessions, session_id: &str) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch, method, js_name = getTabValue)]
    pub async fn get_tab_value(this: &Sessions, tab_id: i32, key: &str) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch, method, js_name = setTabValue)]
    pub async fn set_tab_value(this: &Sessions, tab_id: i32, key: &str, value: &JsValue) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch, method, js_name = removeTabValue)]
    pub async fn remove_tab_value(this: &Sessions, tab_id: i32, key: &str) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch, method, js_name = getWindowValue)]
    pub async fn get_window_value(this: &Sessions, window_id: i32, key: &str) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch, method, js_name = setWindowValue)]
    pub async fn set_window_value(this: &Sessions, window_id: i32, key: &str, value: &JsValue) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch, method, js_name = removeWindowValue)]
    pub async fn remove_window_value(this: &Sessions, window_id: i32, key: &str) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(method, getter, js_name = onChanged)]
    pub fn on_changed(this: &Sessions) -> Event;
}
