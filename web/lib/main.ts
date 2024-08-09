import { WebClient } from "wasm-physics/wasm_physics";
// import { memory } from "wasm-physics/wasm_physics_bg.wasm";

let game: WebClient|null = null;

export const initGame = (canvas: HTMLCanvasElement) => {
  game = new WebClient(canvas);

  canvas.addEventListener("mousedown", () => {
    canvas.focus();
  });

  const loop = () => {
    game!.tick();
    requestAnimationFrame(loop);
  }

  loop();
}

