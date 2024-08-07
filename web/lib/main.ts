import { WebClient } from "wasm-physics/wasm_physics";
// import { memory } from "wasm-physics/wasm_physics_bg.wasm";

let game: WebClient|null = null;

export const initGame = (canvas: HTMLCanvasElement) => {
  game = new WebClient(canvas);
  game.start();

  // const loop = () => {
  //   requestAnimationFrame(loop);
  // }


}

