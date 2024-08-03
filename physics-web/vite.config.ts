// vite.config.js
import { resolve } from 'node:path';
import { defineConfig } from 'vite';
import tsConfigPaths from 'vite-tsconfig-paths';
import solid from 'vite-plugin-solid';
import wasm from 'vite-plugin-wasm';
import dts from 'vite-plugin-dts';

import pkg from "./package.json";

export default defineConfig({
  plugins: [
    solid(),
    wasm(),
    tsConfigPaths(),
    dts({ rollupTypes: true }),
  ],
  build: {
    lib: {
      formats: ["es"],
      entry: resolve(__dirname, 'lib/main.ts')
    },
    rollupOptions: {
      // dependencies will be installed by the consumer,
      // so tell rollup not to bundle them with the package
      external: [
        "wasm-physics/wasm_physics",
        "wasm-physics/wasm_physics_bg.wasm",
        ...Object.keys(pkg["dependencies"]     || {}),
        ...Object.keys(pkg["peerDependencies"] || {}),
        ...Object.keys(pkg["devDependencies"]  || {}),
      ],
    },
  },
})