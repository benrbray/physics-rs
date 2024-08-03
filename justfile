build-wasm:
  cd physics-rs && wasm-pack build

build-web:
  cd physics-web && pnpm run build

build: build-wasm build-web

debug:
  cd physics-rs && wasm-pack build --debug