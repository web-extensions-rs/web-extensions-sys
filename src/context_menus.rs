use js_sys::{Function, Object};
use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::wasm_bindgen;
use crate::EventTarget;

// https://developer.chrome.com/docs/extensions/reference/api/contextMenus
#[wasm_bindgen]
extern "C" {
    pub type ContextMenus;

    #[wasm_bindgen(method)]
    pub fn create(
        this: &ContextMenus,
        create_properties: &Object,
        callback: Option<&Function>,
    ) -> JsValue;

    #[wasm_bindgen(method)]
    pub fn remove(
        this: &ContextMenus,
        menu_item_id: &JsValue,
        callback: Option<&Function>
    );

    #[wasm_bindgen(method, js_name=removeAll)]
    pub fn remove_all(
        this: &ContextMenus,
        callback: Option<&Function>
    );

    #[wasm_bindgen(method)]
    pub fn update(
        this: &ContextMenus,
        id: &JsValue,
        update_properties: &Object,
        callback: Option<&Function>
    );

    #[wasm_bindgen(method, getter, js_name = onClicked)]
    pub fn on_clicked(this: &ContextMenus) -> EventTarget;
}

#[wasm_bindgen]
extern "C" {
    // https://developer.chrome.com/docs/extensions/reference/api/contextMenus#type-OnClickData
    #[derive(Debug)]
    pub type OnClickData;

    // A flag indicating the state of a checkbox or radio item after it is clicked.
    #[wasm_bindgen(method, getter)]
    pub fn checked(this: &OnClickData) -> Option<bool>;

    // A flag indicating whether the element is editable (text input, textarea, etc.).
    #[wasm_bindgen(method, getter)]
    pub fn editable(this: &OnClickData) -> bool;

    // The ID of the frame of the element where the context menu was clicked, if it was in a frame.
    #[wasm_bindgen(method, getter, js_name = frameId)]
    pub fn frame_id(this: &OnClickData) -> Option<u32>;

    // The URL of the frame of the element where the context menu was clicked, if it was in a frame.
    #[wasm_bindgen(method, getter, js_name = frameUrl)]
    pub fn frame_url(this: &OnClickData) -> Option<String>;

    // If the element is a link, the URL it points to.
    #[wasm_bindgen(method, getter, js_name = linkUrl)]
    pub fn link_url(this: &OnClickData) -> Option<String>;

    // One of 'image', 'video', or 'audio' if the context menu was activated on one of these types of elements.
    #[wasm_bindgen(method, getter, js_name = mediaType)]
    pub fn media_type(this: &OnClickData) -> Option<String>;

    // The ID of the menu item that was clicked.
    #[wasm_bindgen(method, getter, js_name = menuItemId)]
    pub fn menu_item_id(this: &OnClickData) -> Option<String>;

    // The URL of the page where the menu item was clicked. This property is not set if the click occured in a context where there is no current page, such as in a launcher context menu.
    #[wasm_bindgen(method, getter, js_name = pageUrl)]
    pub fn page_url(this: &OnClickData) -> Option<String>;

    // The parent ID, if any, for the item clicked.
    #[wasm_bindgen(method, getter, js_name = parentMenuItemId)]
    pub fn parent_menu_item_id(this: &OnClickData) -> Option<String>;

    // The text for the context selection, if any.
    #[wasm_bindgen(method, getter, js_name = selectionText)]
    pub fn selection_text(this: &OnClickData) -> Option<String>;

    // Will be present for elements with a 'src' URL.
    #[wasm_bindgen(method, getter, js_name = srcUrl)]
    pub fn src_url(this: &OnClickData) -> Option<String>;

    // A flag indicating the state of a checkbox or radio item before it was clicked.
    #[wasm_bindgen(method, getter, js_name = wasChecked)]
    pub fn was_checked(this: &OnClickData) -> Option<bool>;
}