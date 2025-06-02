build-crate:
  cd crate && wasm-pack build

build-web:
  cd web && pnpm run build

build: build-crate build-web

build-debug:
  cd crate && wasm-pack build --debug

dev-web:
  cd web && pnpm run dev

dev-debug: build-debug dev-web

dev: build-crate dev-web

release-major:
  cd web && mkdir .git && npm version major && rm -f .git
  git push --follow-tags

release-minor:
  cd web && mkdir .git && npm version minor && rm -f .git
  git push --follow-tags

release-patch:
  cd web && mkdir .git && npm version patch && rm -f .git
  git push --follow-tags