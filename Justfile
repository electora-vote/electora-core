default: build-wasm

build-wasm: build-wasm-web build-wasm-node build-wasm-bundler

build-wasm-web:
    @echo "Building wasm for web"
    wasm-pack build --target web --out-dir ./pkg/web

build-wasm-node:
    @echo "Building wasm for node"
    wasm-pack build --target nodejs --out-dir ./pkg/node

build-wasm-bundler:
    @echo "Building wasm for bundler"
    wasm-pack build --target bundler --out-dir ./pkg/bundler
