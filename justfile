build-crate:
  cd crate && wasm-pack build

build-web:
  cd web && pnpm run build

build: build-crate build-web

debug:
  cd crate && wasm-pack build --debug

dev-web:
  cd web && pnpm run dev

dev: build-crate dev-web