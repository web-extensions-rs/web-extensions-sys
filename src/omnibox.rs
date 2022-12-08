//! Bindings to the `omnibox` API.

use wasm_bindgen::prelude::*;

use crate::EventTarget;

#[wasm_bindgen]
extern "C" {
    pub type Omnibox;

    #[wasm_bindgen(method, js_name = setDefaultSuggestion)]
    pub fn set_default_suggestion(this: &Omnibox, suggestion: &JsValue);

    #[wasm_bindgen(method, getter, js_name = onDeleteSuggestion)]
    pub fn on_delete_suggestion(this: &Omnibox) -> EventTarget;

    #[wasm_bindgen(method, getter, js_name = onInputCancelled)]
    pub fn on_input_cancelled(this: &Omnibox) -> EventTarget;

    #[wasm_bindgen(method, getter, js_name = onInputChanged)]
    pub fn on_input_changed(this: &Omnibox) -> EventTarget;

    #[wasm_bindgen(method, getter, js_name = onInputEntered)]
    pub fn on_input_entered(this: &Omnibox) -> EventTarget;

    #[wasm_bindgen(method, getter, js_name = onInputStarted)]
    pub fn on_input_started(this: &Omnibox) -> EventTarget;
}
