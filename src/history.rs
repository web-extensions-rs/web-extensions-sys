//! Bindings to the `history` API.

use js_sys::{Number, Object};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    pub type History;

    #[wasm_bindgen(method)]
    pub async fn search(this: &History, query: &Object) -> JsValue;
}

#[wasm_bindgen]
extern "C" {

    // An object encapsulating one result of a history query.
    pub type HistoryItem;

    // The unique identifier for the item.
    #[wasm_bindgen(method, getter)]
    pub fn id(this: &HistoryItem) -> String;

    // When this page was last loaded, represented in milliseconds since the epoch.
    #[wasm_bindgen(method, getter, js_name = lastVisitTime)]
    pub fn last_visit_time(this: &HistoryItem) -> Option<Number>;

    // The title of the page when it was last loaded.
    #[wasm_bindgen(method, getter)]
    pub fn title(this: &HistoryItem) -> Option<String>;

    // The number of times the user has navigated to this page by typing in the address.
    #[wasm_bindgen(method, getter, js_name = typedCount)]
    pub fn typed_count(this: &HistoryItem) -> Option<Number>;

    // The URL navigated to by a user.
    #[wasm_bindgen(method, getter)]
    pub fn url(this: &HistoryItem) -> Option<String>;

    // The number of times the user has navigated to this page.
    #[wasm_bindgen(method, getter, js_name = visitCount)]
    pub fn visit_count(this: &HistoryItem) -> Option<Number>;
}
