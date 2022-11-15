# Web Extensions (sys)

A Rust library that provides
[WebExtension API](https://developer.chrome.com/docs/extensions/reference/)
[WASM](https://en.wikipedia.org/wiki/WebAssembly) bindings.

This crate expresses a low level wrapper.
For a higher level abstraction there is the
[`web-extensions`](https://github.com/web-extensions-rs/web-extensions)
crate.

## Compatibility

This library is currently only compatible with Chrome based browsers
with [Manifest V3](https://developer.chrome.com/docs/extensions/mv3/intro/).
