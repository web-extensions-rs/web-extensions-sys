//! Bindings to the `bookmarks` API.

use js_sys::{Array, Number};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    pub type Bookmarks;

    #[wasm_bindgen(method)]
    async fn search(this: &Bookmarks, query: &JsValue) -> JsValue;
}

#[wasm_bindgen]
extern "C" {

    // A node (either a bookmark or a folder) in the bookmark tree.
    pub type BookmarkTreeNode;

    // An ordered list of children of this node.
    #[wasm_bindgen(method)]
    pub fn children(this: &BookmarkTreeNode) -> Option<Array>;

    // When this node was created, in milliseconds since the epoch (new Date(dateAdded)).
    #[wasm_bindgen(method, js_name = dateAdded)]
    pub fn date_added(this: &BookmarkTreeNode) -> Option<Number>;

    // When the contents of this folder last changed, in milliseconds since the epoch.
    #[wasm_bindgen(method, js_name = dateGroupModified)]
    pub fn date_group_modified(this: &BookmarkTreeNode) -> Option<Number>;

    // The unique identifier for the node. IDs are unique within the current profile, and they remain valid even after the browser is restarted.
    #[wasm_bindgen(method)]
    pub fn id(this: &BookmarkTreeNode) -> String;

    // The 0-based position of this node within its parent folder.
    #[wasm_bindgen(method)]
    pub fn index(this: &BookmarkTreeNode) -> Option<Number>;

    // The id of the parent folder. Omitted for the root node.
    #[wasm_bindgen(method, js_name = parentId)]
    pub fn parent_id(this: &BookmarkTreeNode) -> Option<String>;

    // The text displayed for the node.
    #[wasm_bindgen(method)]
    pub fn title(this: &BookmarkTreeNode) -> String;

    // Indicates the reason why this node is unmodifiable.
    #[wasm_bindgen(method)]
    pub fn unmodifiable(this: &BookmarkTreeNode) -> Option<String>;

    // The URL navigated to when a user clicks the bookmark. Omitted for folders.
    #[wasm_bindgen(method)]
    pub fn url(this: &BookmarkTreeNode) -> Option<String>;

}
