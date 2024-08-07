import { init_game } from "wasm-physics/wasm_physics";
// import { memory } from "wasm-physics/wasm_physics_bg.wasm";


export const initGame = (canvas: HTMLCanvasElement) => {
  init_game(canvas);
}

// export const initGameOfLife = (canvas: HTMLCanvasElement) => {
//   const universe = Universe.new(64, 64);
//   const width = universe.width();
//   const height = universe.height();


//   const ctx = canvas.getContext("2d");
//   if(!ctx) { throw new Error("missing canvas context"); }

//   // resize canvas
//   canvas.height = (CELL_SIZE + 1) * height + 1;
//   canvas.width = (CELL_SIZE + 1) * width + 1;

//   const render = () => {
//     drawGrid(ctx, width, height);
//     drawCells(ctx, universe, width, height);
//   }

//   const renderLoop = () => {
//     universe.tick();

//     render();

//     requestAnimationFrame(renderLoop);
//   }

//   render();
//   requestAnimationFrame(renderLoop);
// }

export const foo = 5;