use gloo_console as console;
use gloo_utils::{body, document};
use wasm_bindgen::{prelude::*, JsCast};
use web_extensions_sys::{chrome, Port};

#[wasm_bindgen]
pub fn start() {
    console::info!("Start foreground script");
    render_container();
    let port = connect();

    let on_message = |msg: JsValue| {
        console::info!("Received message:", msg);
    };
    let closure: Closure<dyn Fn(JsValue)> = Closure::new(on_message);
    let callback = closure.as_ref().unchecked_ref();
    port.on_message().add_listener(callback);
    closure.forget();
    let msg = JsValue::from_serde(&messages::PortRequest::Ping).unwrap();
    port.post_message(&msg);
}

fn render_container() {
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

fn connect() -> Port {
    let connect_info = JsValue::null();
    chrome
        .runtime()
        .connect(None, connect_info.as_ref().unchecked_ref())
}
