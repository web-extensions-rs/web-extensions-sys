# Basic Web Extension

A basic Web Extension for Google Chrome (version `>= 102`) written in Rust.

## Build

```
cd background-script/ && wasm-pack build --release -t web
cd ../
cd foreground-script/ && wasm-pack build --release -t no-modules
```

## Run

Navigate to `chrome://extensions/` in the browser and activate the developer mode.
Then you can load the unpacked extension
(select the folder `examples/basic-web-extensions`).
