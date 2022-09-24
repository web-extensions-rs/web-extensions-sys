use crate::{EventTarget, Port};
use js_sys::Object;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    pub type Runtime;

    #[wasm_bindgen(catch, method, js_name = sendMessage)]
    pub async fn send_message(
        this: &Runtime,
        extension_id: Option<&str>,
        message: &JsValue,
        options: Option<&Object>,
    ) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(method)]
    pub fn connect(this: &Runtime, extension_id: Option<&str>, connect_info: &Object) -> Port;

    #[wasm_bindgen(method, getter, js_name = onMessage)]
    pub fn on_message(this: &Runtime) -> EventTarget;

    #[wasm_bindgen(method, getter, js_name = onConnect)]
    pub fn on_connect(this: &Runtime) -> EventTarget;

    #[wasm_bindgen(method, getter, js_name = onInstalled)]
    pub fn on_installed(this: &Runtime) -> EventTarget;

    #[wasm_bindgen(method, js_name = setUninstallURL)]
    pub fn set_uninstall_url(this: &Runtime, url: &str);

    #[wasm_bindgen(method, js_name = openOptionsPage)]
    pub fn open_options_page(this: &Runtime);

}
