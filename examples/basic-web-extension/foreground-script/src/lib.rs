use gloo_console as console;
use gloo_utils::{body, document};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn start() {
    console::info!("Start foreground script");
    let container = document().create_element("div").unwrap();
    container
        .class_list()
        .add_1("wea-example-container")
        .unwrap();

    let title = document().create_element("h2").unwrap();
    title.set_inner_html("Example Web Extension Foreground");

    let data = document().create_element("div").unwrap();
    data.set_inner_html("Hello from foreground script");

    container.append_child(&title).unwrap();
    container.append_child(&data).unwrap();
    body().append_child(&container).unwrap();
}
