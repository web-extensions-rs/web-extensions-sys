# just manual: https://github.com/casey/just/#readme

set windows-shell := ["pwsh.exe", "-NoLogo", "-Command"]

_default:
    @just --list

# Build the web extension
build:
  cd background-script/ && wasm-pack build --release -t web
  cd foreground-script/ && wasm-pack build --release -t no-modules
  cd options/ && wasm-pack build --release -t web
