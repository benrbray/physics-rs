import { resolve } from 'node:path';

// vite.config.js
import { Plugin, defineConfig, searchForWorkspaceRoot } from 'vite';

// vite plugins
import tsConfigPaths from 'vite-tsconfig-paths';
import solid from 'vite-plugin-solid';
import wasm from 'vite-plugin-wasm';
import dts from 'vite-plugin-dts';

// package
import pkg from "./package.json";

////////////////////////////////////////////////////////////////////////////////

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
        // "wasm-physics/wasm_physics",
        // "wasm-physics/wasm_physics_bg.wasm",
        ...Object.keys(pkg["dependencies"]     || {}),
        ...Object.keys(pkg["peerDependencies"] || {}),
        ...Object.keys(pkg["devDependencies"]  || {}),
      ],
    },
  },
  // server: {
  //   headers: {
  //     // required for `SharedArrayBuffer`
  //     "Cross-Origin-Embedder-Policy": "require-corp",
  //     "Cross-Origin-Opener-Policy": "same-origin",
  //   },
  //   fs: {
  //     allow: [
  //       // preserve default server behavior
  //       searchForWorkspaceRoot(process.cwd()),
  //       // allow exports from crate/pkg
  //       '..',
  //     ]
  //   }
  // }
})