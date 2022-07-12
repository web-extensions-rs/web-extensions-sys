use crate::EventTarget;
use js_sys::{Array, Promise};
use wasm_bindgen::{prelude::*, JsCast};

#[wasm_bindgen]
extern "C" {
    #[derive(Debug, Clone)]
    pub type ThemeImages;

    #[wasm_bindgen(method, getter)]
    pub fn theme_frame(this: &ThemeImages) -> Option<String>;
}

#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    pub type Color;
}

impl Color {
    #[allow(clippy::should_implement_trait)]
    pub fn from_str(color: &str) -> Self {
        JsValue::from(color).unchecked_into()
    }

    pub fn from_rgb(array: [u8; 3]) -> Self {
        Array::of3(
            &JsValue::from(array[0]),
            &JsValue::from(array[1]),
            &JsValue::from(array[2]),
        )
        .unchecked_into()
    }

    pub fn as_string(&self) -> Option<String> {
        if let Some(string) = self.unchecked_ref::<JsValue>().as_string() {
            Some(string)
        } else if let Some(array) = self.dyn_ref::<Array>() {
            // Convert RGB Array into String
            let r = array.get(0).as_f64()?;
            let g = array.get(1).as_f64()?;
            let b = array.get(2).as_f64()?;
            Some(format!("rgb({}, {}, {})", r / 255.0, g / 255.0, b / 255.0))
        } else {
            None
        }
    }

    fn as_u8(value: JsValue) -> Option<u8> {
        let float = value.as_f64()?;
        if float.fract() == 0.0 && float >= 0.0 && float <= 255.0 {
            Some(float as u8)
        } else {
            None
        }
    }

    pub fn as_rgb(&self) -> Option<[u8; 3]> {
        if let Some(array) = self.dyn_ref::<Array>() {
            let r = Self::as_u8(array.get(0))?;
            let g = Self::as_u8(array.get(1))?;
            let b = Self::as_u8(array.get(2))?;
            Some([r, g, b])
        } else {
            None
        }
    }
}

#[wasm_bindgen]
extern "C" {
    #[derive(Debug, Clone)]
    pub type ThemeColors;

    #[wasm_bindgen(method, getter)]
    pub fn button_background_active(this: &ThemeColors) -> Color;

    #[wasm_bindgen(method, getter)]
    pub fn button_background_hover(this: &ThemeColors) -> Color;

    #[wasm_bindgen(method, getter)]
    pub fn icons(this: &ThemeColors) -> Color;

    #[wasm_bindgen(method, getter)]
    pub fn icons_attention(this: &ThemeColors) -> Color;

    #[wasm_bindgen(method, getter)]
    pub fn frame(this: &ThemeColors) -> Color;

    #[wasm_bindgen(method, getter)]
    pub fn frame_inactive(this: &ThemeColors) -> Color;

    #[wasm_bindgen(method, getter)]
    pub fn ntp_background(this: &ThemeColors) -> Color;

    #[wasm_bindgen(method, getter)]
    pub fn ntp_text(this: &ThemeColors) -> Color;

    #[wasm_bindgen(method, getter)]
    pub fn popup(this: &ThemeColors) -> Color;

    #[wasm_bindgen(method, getter)]
    pub fn popup_border(this: &ThemeColors) -> Color;

    #[wasm_bindgen(method, getter)]
    pub fn popup_highlight(this: &ThemeColors) -> Color;

    #[wasm_bindgen(method, getter)]
    pub fn popup_highlight_text(this: &ThemeColors) -> Color;

    #[wasm_bindgen(method, getter)]
    pub fn popup_text(this: &ThemeColors) -> Color;

    #[wasm_bindgen(method, getter)]
    pub fn sidebar(this: &ThemeColors) -> Color;

    #[wasm_bindgen(method, getter)]
    pub fn sidebar_border(this: &ThemeColors) -> Color;

    #[wasm_bindgen(method, getter)]
    pub fn sidebar_highlight(this: &ThemeColors) -> Color;

    #[wasm_bindgen(method, getter)]
    pub fn sidebar_highlight_text(this: &ThemeColors) -> Color;

    #[wasm_bindgen(method, getter)]
    pub fn sidebar_text(this: &ThemeColors) -> Color;

    #[wasm_bindgen(method, getter)]
    pub fn tab_background_separator(this: &ThemeColors) -> Color;

    #[wasm_bindgen(method, getter)]
    pub fn tab_background_text(this: &ThemeColors) -> Color;

    #[wasm_bindgen(method, getter)]
    pub fn tab_line(this: &ThemeColors) -> Color;

    #[wasm_bindgen(method, getter)]
    pub fn tab_loading(this: &ThemeColors) -> Color;

    #[wasm_bindgen(method, getter)]
    pub fn tab_selected(this: &ThemeColors) -> Color;

    #[wasm_bindgen(method, getter)]
    pub fn tab_text(this: &ThemeColors) -> Color;

    #[wasm_bindgen(method, getter)]
    pub fn toolbar(this: &ThemeColors) -> Color;

    #[wasm_bindgen(method, getter)]
    pub fn toolbar_bottom_separator(this: &ThemeColors) -> Color;

    #[wasm_bindgen(method, getter)]
    pub fn toolbar_field(this: &ThemeColors) -> Color;

    #[wasm_bindgen(method, getter)]
    pub fn toolbar_field_border(this: &ThemeColors) -> Color;

    #[wasm_bindgen(method, getter)]
    pub fn toolbar_field_border_focus(this: &ThemeColors) -> Color;

    #[wasm_bindgen(method, getter)]
    pub fn toolbar_field_focus(this: &ThemeColors) -> Color;

    #[wasm_bindgen(method, getter)]
    pub fn toolbar_field_highlight(this: &ThemeColors) -> Color;

    #[wasm_bindgen(method, getter)]
    pub fn toolbar_field_highlight_text(this: &ThemeColors) -> Color;

    #[wasm_bindgen(method, getter)]
    pub fn toolbar_field_separator(this: &ThemeColors) -> Color;

    #[wasm_bindgen(method, getter)]
    pub fn toolbar_field_text(this: &ThemeColors) -> Color;

    #[wasm_bindgen(method, getter)]
    pub fn toolbar_field_text_focus(this: &ThemeColors) -> Color;

    #[wasm_bindgen(method, getter)]
    pub fn toolbar_text(this: &ThemeColors) -> Color;

    #[wasm_bindgen(method, getter)]
    pub fn toolbar_top_separator(this: &ThemeColors) -> Color;

    #[wasm_bindgen(method, getter)]
    pub fn toolbar_vertical_separator(this: &ThemeColors) -> Color;
}

#[wasm_bindgen]
extern "C" {
    #[derive(Debug, Clone)]
    pub type ThemeProperties;

    #[wasm_bindgen(method, getter)]
    pub fn additional_backgrounds_alignment(this: &ThemeProperties) -> Option<Array>;

    #[wasm_bindgen(method, getter)]
    pub fn additional_backgrounds_tiling(this: &ThemeProperties) -> Option<Array>;
}

#[wasm_bindgen]
extern "C" {
    #[derive(Debug, Clone)]
    pub type Theme;

    #[wasm_bindgen(method, getter)]
    pub fn images(this: &Theme) -> Option<ThemeImages>;

    #[wasm_bindgen(method, getter)]
    pub fn colors(this: &Theme) -> Option<ThemeColors>;

    #[wasm_bindgen(method, getter)]
    pub fn properties(this: &Theme) -> Option<ThemeProperties>;
}

#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    pub type ThemeUpdateInfo;

    #[wasm_bindgen(method, getter)]
    pub fn theme(this: &ThemeUpdateInfo) -> Theme;

    #[wasm_bindgen(method, getter, js_name = windowId)]
    pub fn window_id(this: &ThemeUpdateInfo) -> Option<i32>;
}

#[wasm_bindgen]
extern "C" {
    pub type BrowserTheme;

    #[wasm_bindgen(method, js_name = getCurrent)]
    pub fn get_current(this: &BrowserTheme, window_id: Option<i32>) -> Promise;

    #[wasm_bindgen(method)]
    pub fn update(this: &BrowserTheme, window_id: Option<i32>, theme: &Theme);

    #[wasm_bindgen(method)]
    pub fn reset(this: &BrowserTheme, window_id: Option<i32>);

    #[wasm_bindgen(method, getter, js_name = onUpdated)]
    pub fn on_updated(this: &BrowserTheme) -> EventTarget;
}
