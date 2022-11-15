// https://developer.chrome.com/docs/extensions/reference/tabs/

use crate::EventTarget;
use js_sys::Object;
use wasm_bindgen::prelude::*;

/// The tab's ID.
///
/// Tab IDs are unique within a browser session.
type TabId = i32; // `TAB_ID_NONE` has value `-1` so we have to use i32

/// The ID of the window that hosts a tab.
type WindowId = i32;

/// The ID of the group that the tab belongs to.
type GroupId = i32;

/// Zero-based index of the tab within its window.
type TabIndex = u32;

#[wasm_bindgen]
extern "C" {
    // https://developer.chrome.com/docs/extensions/reference/tabs/#type-onActivated-callback-activeInfo
    #[derive(Debug)]
    pub type TabActiveInfo;

    #[wasm_bindgen(method, getter, js_name = previousTabId)]
    pub fn previous_tab_id(this: &TabActiveInfo) -> Option<TabId>;

    #[wasm_bindgen(method, getter, js_name = tabId)]
    pub fn tab_id(this: &TabActiveInfo) -> TabId;

    #[wasm_bindgen(method, getter, js_name = windowId)]
    pub fn window_id(this: &TabActiveInfo) -> WindowId;
}

#[wasm_bindgen]
extern "C" {
    // https://developer.chrome.com/docs/extensions/reference/tabs/#type-onDetached-callback-detachInfo
    #[derive(Debug)]
    pub type TabDetachInfo;

    #[wasm_bindgen(method, getter, js_name = oldWindowId)]
    pub fn old_window_id(this: &TabDetachInfo) -> WindowId;

    #[wasm_bindgen(method, getter, js_name = oldPosition)]
    pub fn old_position(this: &TabDetachInfo) -> TabIndex;
}

#[wasm_bindgen]
extern "C" {
    // https://developer.chrome.com/docs/extensions/reference/tabs/#type-onAttached-callback-attachInfo
    #[derive(Debug)]
    pub type TabAttachInfo;

    #[wasm_bindgen(method, getter, js_name = newWindowId)]
    pub fn new_window_id(this: &TabAttachInfo) -> WindowId;

    #[wasm_bindgen(method, getter, js_name = newPosition)]
    pub fn new_position(this: &TabAttachInfo) -> TabIndex;
}

#[wasm_bindgen]
extern "C" {
    // https://developer.chrome.com/docs/extensions/reference/tabs/#type-onMoved-callback-moveInfo
    #[derive(Debug)]
    pub type TabMoveInfo;

    #[wasm_bindgen(method, getter, js_name = windowId)]
    pub fn window_id(this: &TabMoveInfo) -> WindowId;

    #[wasm_bindgen(method, getter, js_name = fromIndex)]
    pub fn from_index(this: &TabMoveInfo) -> TabIndex;

    #[wasm_bindgen(method, getter, js_name = toIndex)]
    pub fn to_index(this: &TabMoveInfo) -> TabIndex;
}

#[wasm_bindgen]
extern "C" {
    // https://developer.chrome.com/docs/extensions/reference/tabs/#type-onRemoved-callback-removeInfo
    #[derive(Debug)]
    pub type TabRemoveInfo;

    #[wasm_bindgen(method, getter, js_name = windowId)]
    pub fn window_id(this: &TabRemoveInfo) -> WindowId;

    #[wasm_bindgen(method, getter, js_name = isWindowClosing)]
    pub fn is_window_closing(this: &TabRemoveInfo) -> bool;
}

#[wasm_bindgen]
extern "C" {
    // https://developer.chrome.com/docs/extensions/reference/tabs/#type-MutedInfo
    #[derive(Debug)]
    pub type TabMutedInfo;

    #[wasm_bindgen(method, getter)]
    pub fn muted(this: &TabMutedInfo) -> bool;

    #[wasm_bindgen(method, getter, js_name = extensionId)]
    pub fn extension_id(this: &TabMutedInfo) -> Option<String>;

    #[wasm_bindgen(method, getter)]
    pub fn reason(this: &TabMutedInfo) -> Option<String>;
}

#[wasm_bindgen]
extern "C" {
    // https://developer.chrome.com/docs/extensions/reference/tabs/#type-Tab
    #[derive(Debug, Clone)]
    pub type Tab;

    #[wasm_bindgen(method, getter)]
    pub fn active(this: &Tab) -> bool;

    #[wasm_bindgen(method, getter)]
    pub fn audible(this: &Tab) -> Option<bool>;

    #[cfg(not(feature = "firefox"))]
    #[wasm_bindgen(method, getter, js_name = autoDiscardable)]
    pub fn auto_discardable(this: &Tab) -> bool;

    #[cfg(feature = "firefox")]
    #[wasm_bindgen(method, getter, js_name = autoDiscardable)]
    pub fn auto_discardable(this: &Tab) -> Option<bool>;

    #[cfg(not(feature = "firefox"))]
    #[wasm_bindgen(method, getter)]
    pub fn discarded(this: &Tab) -> bool;

    #[cfg(feature = "firefox")]
    #[wasm_bindgen(method, getter)]
    pub fn discarded(this: &Tab) -> Option<bool>;

    #[wasm_bindgen(method, getter, js_name = favIconUrl)]
    pub fn fav_icon_url(this: &Tab) -> Option<String>;

    #[cfg(not(feature = "firefox"))]
    #[wasm_bindgen(method, getter, js_name = groupId)]
    pub fn group_id(this: &Tab) -> GroupId;

    #[cfg(feature = "firefox")]
    #[wasm_bindgen(method, getter, js_name = groupId)]
    pub fn group_id(this: &Tab) -> Option<GroupId>;

    #[wasm_bindgen(method, getter)]
    pub fn height(this: &Tab) -> Option<u32>;

    #[wasm_bindgen(method, getter)]
    pub fn highlighted(this: &Tab) -> bool;

    #[wasm_bindgen(method, getter)]
    pub fn id(this: &Tab) -> Option<TabId>;

    #[wasm_bindgen(method, getter)]
    pub fn incognito(this: &Tab) -> bool;

    #[wasm_bindgen(method, getter)]
    pub fn index(this: &Tab) -> TabIndex;

    #[cfg(not(feature = "firefox"))]
    #[wasm_bindgen(method, getter, js_name = mutedInfo)]
    pub fn muted_info(this: &Tab) -> Option<TabMutedInfo>;

    #[cfg(feature = "firefox")]
    #[wasm_bindgen(method, getter, js_name = mutedInfo)]
    pub fn muted_info(this: &Tab) -> TabMutedInfo;

    #[wasm_bindgen(method, getter, js_name = openerTabId)]
    pub fn opener_tab_id(this: &Tab) -> Option<TabId>;

    #[wasm_bindgen(method, getter, js_name = pendingUrl)]
    pub fn pending_url(this: &Tab) -> Option<String>;

    #[wasm_bindgen(method, getter)]
    pub fn pinned(this: &Tab) -> bool;

    #[wasm_bindgen(method, getter, js_name = sessionId)]
    pub fn session_id(this: &Tab) -> Option<String>;

    #[wasm_bindgen(method, getter)]
    pub fn status(this: &Tab) -> Option<String>;

    #[wasm_bindgen(method, getter)]
    pub fn title(this: &Tab) -> Option<String>;

    #[wasm_bindgen(method, getter)]
    pub fn url(this: &Tab) -> Option<String>;

    #[wasm_bindgen(method, getter)]
    pub fn width(this: &Tab) -> Option<u32>;

    #[wasm_bindgen(method, getter, js_name = windowId)]
    pub fn window_id(this: &Tab) -> WindowId;

    // --- Firefox only --- //

    #[cfg(feature = "firefox")]
    #[wasm_bindgen(method, getter)]
    pub fn attention(this: &Tab) -> Option<bool>;

    #[cfg(feature = "firefox")]
    #[wasm_bindgen(method, getter, js_name = cookieStoreId)]
    pub fn cookie_store_id(this: &Tab) -> Option<String>;

    #[cfg(feature = "firefox")]
    #[wasm_bindgen(method, getter)]
    pub fn hidden(this: &Tab) -> bool;

    #[cfg(feature = "firefox")]
    #[wasm_bindgen(method, getter, js_name = isArticle)]
    pub fn is_article(this: &Tab) -> bool;

    #[cfg(feature = "firefox")]
    #[wasm_bindgen(method, getter, js_name = isInReaderMode)]
    pub fn is_in_reader_mode(this: &Tab) -> bool;

    #[cfg(feature = "firefox")]
    #[wasm_bindgen(method, getter, js_name = lastAccessed)]
    pub fn last_accessed(this: &Tab) -> f64;

    #[cfg(feature = "firefox")]
    #[wasm_bindgen(method, getter, js_name = successorId)]
    pub fn successor_id(this: &Tab) -> Option<TabId>;

}

#[wasm_bindgen]
extern "C" {
    // https://developer.chrome.com/docs/extensions/reference/tabs/#method
    pub type Tabs;

    #[wasm_bindgen(method, getter, js_name = TAB_ID_NONE)]
    pub fn tab_id_none(this: &Tabs) -> TabId;

    #[wasm_bindgen(catch, method, js_name = captureTab)]
    pub async fn capture_tab(
        this: &Tabs,
        tab_id: Option<TabId>,
        info: Option<&Object>,
    ) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch, method, js_name = captureVisibleTab)]
    pub async fn capture_visible_tab(
        this: &Tabs,
        window_id: Option<WindowId>,
        info: Option<&Object>,
    ) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch, method)]
    pub async fn connect(
        this: &Tabs,
        tab_id: TabId,
        info: Option<&Object>,
    ) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch, method)]
    pub async fn create(this: &Tabs, info: &Object) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch, method)]
    pub async fn discard(this: &Tabs, tab_ids: &JsValue) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch, method)]
    pub async fn duplicate(this: &Tabs, tab_id: TabId) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch, method)]
    pub async fn get(this: &Tabs, tab_id: TabId) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch, method, js_name = getCurrent)]
    pub async fn get_current(this: &Tabs) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch, method, js_name = getZoom)]
    pub async fn get_zoom(this: &Tabs, tab_id: Option<TabId>) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch, method, js_name = getZoomSettings)]
    pub async fn get_zoom_settings(this: &Tabs, tab_id: Option<TabId>) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch, method)]
    pub async fn hide(this: &Tabs, tab_ids: &JsValue) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch, method)]
    pub async fn highlight(this: &Tabs, info: &Object) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch, method, js_name = insertCSS)]
    pub async fn insert_css(
        this: &Tabs,
        tab_id: Option<TabId>,
        info: &Object,
    ) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch, method, js_name = move)]
    pub async fn move_(this: &Tabs, tab_ids: &JsValue, info: &Object) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch, method, js_name = moveInSuccession)]
    pub async fn move_in_succession(
        this: &Tabs,
        tab_ids: &JsValue,
        tab_id: Option<TabId>,
        info: Option<&Object>,
    ) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(method)]
    pub fn print(this: &Tabs);

    #[wasm_bindgen(catch, method, js_name = printPreview)]
    pub async fn print_preview(this: &Tabs) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch, method)]
    pub async fn query(this: &Tabs, info: &Object) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch, method)]
    pub async fn reload(
        this: &Tabs,
        tab_id: Option<TabId>,
        info: Option<&Object>,
    ) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch, method)]
    pub async fn remove(this: &Tabs, tab_ids: &JsValue) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch, method, js_name = removeCSS)]
    pub async fn remove_css(
        this: &Tabs,
        tab_id: Option<TabId>,
        info: &Object,
    ) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch, method, js_name = saveAsPDF)]
    pub async fn save_as_pdf(this: &Tabs, info: &Object) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch, method, js_name = sendMessage)]
    pub async fn send_message(
        this: &Tabs,
        tab_id: TabId,
        message: &JsValue,
        info: Option<&Object>,
    ) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch, method, js_name = setZoom)]
    pub async fn set_zoom(
        this: &Tabs,
        tab_id: Option<TabId>,
        zoom_factor: f64,
    ) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch, method, js_name = setZoomSettings)]
    pub async fn set_zoom_settings(
        this: &Tabs,
        tab_id: Option<TabId>,
        info: &Object,
    ) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch, method)]
    pub async fn show(this: &Tabs, tab_ids: &JsValue) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch, method, js_name = toggleReaderMode)]
    pub async fn toggle_reader_mode(this: &Tabs, tab_id: Option<TabId>)
        -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch, method)]
    pub async fn update(
        this: &Tabs,
        tab_id: Option<TabId>,
        info: &Object,
    ) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch, method, js_name = detectLanguage)]
    pub async fn detect_language(this: &Tabs, tab_id: Option<TabId>) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(method, getter, js_name = onActivated)]
    pub fn on_activated(this: &Tabs) -> EventTarget;

    #[wasm_bindgen(method, getter, js_name = onAttached)]
    pub fn on_attached(this: &Tabs) -> EventTarget;

    #[wasm_bindgen(method, getter, js_name = onCreated)]
    pub fn on_created(this: &Tabs) -> EventTarget;

    #[wasm_bindgen(method, getter, js_name = onDetached)]
    pub fn on_detached(this: &Tabs) -> EventTarget;

    #[wasm_bindgen(method, getter, js_name = onHighlighted)]
    pub fn on_highlighted(this: &Tabs) -> EventTarget;

    #[wasm_bindgen(method, getter, js_name = onMoved)]
    pub fn on_moved(this: &Tabs) -> EventTarget;

    #[wasm_bindgen(method, getter, js_name = onRemoved)]
    pub fn on_removed(this: &Tabs) -> EventTarget;

    #[wasm_bindgen(method, getter, js_name = onReplaced)]
    pub fn on_replaced(this: &Tabs) -> EventTarget;

    #[wasm_bindgen(method, getter, js_name = onUpdated)]
    pub fn on_updated(this: &Tabs) -> EventTarget;

    #[wasm_bindgen(method, getter, js_name = onZoomChange)]
    pub fn on_zoom_change(this: &Tabs) -> EventTarget;
}

#[wasm_bindgen]
extern "C" {
    // https://developer.chrome.com/docs/extensions/reference/tabs/#type-onUpdated-callback-changeInfo
    #[derive(Debug)]
    pub type TabChangeInfo;

    // The tab's new audible state.
    #[wasm_bindgen(method, getter)]
    pub fn audible(this: &TabChangeInfo) -> Option<bool>;

    // The tab's new auto-discardable state.
    #[wasm_bindgen(method, getter, js_name = autoDiscardable)]
    pub fn auto_discardable(this: &TabChangeInfo) -> Option<bool>;

    // The tab's new discarded state.
    #[wasm_bindgen(method, getter)]
    pub fn discarded(this: &TabChangeInfo) -> Option<bool>;

    // The tab's new favicon URL.
    #[wasm_bindgen(method, getter, js_name = favIconUrl)]
    pub fn fav_icon_url(this: &TabChangeInfo) -> Option<String>;

    // The tab's new group.
    #[wasm_bindgen(method, getter, js_name = groupId)]
    pub fn group_id(this: &TabChangeInfo) -> Option<GroupId>;

    // The tab's new muted state and the reason for the change.
    #[wasm_bindgen(method, getter, js_name = mutedInfo)]
    pub fn muted_info(this: &TabChangeInfo) -> Option<TabMutedInfo>;

    // The tab's new pinned state.
    #[wasm_bindgen(method, getter)]
    pub fn pinned(this: &TabChangeInfo) -> Option<bool>;

    // The tab's loading status.
    #[wasm_bindgen(method, getter)]
    pub fn status(this: &TabChangeInfo) -> Option<String>;

    // The tab's new title.
    #[wasm_bindgen(method, getter)]
    pub fn title(this: &TabChangeInfo) -> Option<String>;

    // The tab's URL if it has changed.
    #[wasm_bindgen(method, getter)]
    pub fn url(this: &TabChangeInfo) -> Option<String>;
}

#[wasm_bindgen]
extern "C" {
    // https://developer.chrome.com/docs/extensions/reference/tabs/#type-onHighlighted-callback-highlightInfo
    #[derive(Debug)]
    pub type TabHighlightInfo;

    // All highlighted tabs in the window.
    #[wasm_bindgen(method, getter, js_name = tabIds)]
    pub fn tab_ids(this: &TabHighlightInfo) -> JsValue;

    // The window whose tabs changed.
    #[wasm_bindgen(method, getter, js_name = windowId)]
    pub fn window_id(this: &TabHighlightInfo) -> WindowId;
}

#[wasm_bindgen]
extern "C" {
    // https://developer.chrome.com/docs/extensions/reference/tabs/#type-onZoomChange-callback-ZoomChangeInfo
    #[derive(Debug)]
    pub type TabZoomChangeInfo;

    #[wasm_bindgen(method, getter, js_name = newZoomFactor)]
    pub fn new_zoom_factor(this: &TabZoomChangeInfo) -> f64;

    #[wasm_bindgen(method, getter, js_name = oldZoomFactor)]
    pub fn old_zoom_factor(this: &TabZoomChangeInfo) -> f64;

    #[wasm_bindgen(method, getter, js_name = tabId)]
    pub fn tab_id(this: &TabZoomChangeInfo) -> TabId;

    #[wasm_bindgen(method, getter, js_name = zoomSettings)]
    pub fn zoom_settings(this: &TabZoomChangeInfo) -> JsValue;
}
