//! Authorization via OAuth2.

use js_sys::Function;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {

    pub type Identity;

    #[wasm_bindgen(method, js_name = getRedirectURL)]
    pub fn get_redirect_url(this: &Identity) -> String;

    #[wasm_bindgen(method, js_name = getRedirectURL)]
    pub fn get_redirect_url_with_path(this: &Identity, path: &str) -> String;

    #[wasm_bindgen(method, catch, js_name = launchWebAuthFlow)]
    pub async fn launch_webauth_flow(
        this: &Identity,
        details: &JsValue,
    ) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(method, catch, js_name = launchWebAuthFlow)]
    pub fn launch_webauth_flow_with_callback(
        this: &Identity,
        details: &JsValue,
        callback: &Function,
    ) -> Result<(), JsValue>;

    #[wasm_bindgen(method, catch, js_name = getAuthToken)]
    pub fn get_auth_token(this: &Identity) -> Result<(), JsValue>;

    #[wasm_bindgen(method, catch, js_name = getAuthToken)]
    pub fn get_auth_token_with_details(this: &Identity, details: &JsValue) -> Result<(), JsValue>;

    #[wasm_bindgen(method, catch, js_name = getAuthToken)]
    pub fn get_auth_token_with_details_and_callback(
        this: &Identity,
        details: &JsValue,
        callback: &Function,
    ) -> Result<(), JsValue>;
}
