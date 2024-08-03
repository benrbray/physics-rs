import { Universe, Cell, init_webgl } from "wasm-physics/wasm_physics";
import { memory } from "wasm-physics/wasm_physics_bg.wasm";

const CELL_SIZE = 5; // px
const GRID_COLOR = "#CCCCCC";
const DEAD_COLOR = "#FFFFFF";
const ALIVE_COLOR = "#000000";

const drawGrid = (
  ctx: CanvasRenderingContext2D,
  width: number,
  height: number
) => {
  ctx.beginPath();
  ctx.strokeStyle = GRID_COLOR;

  // Vertical lines.
  for (let i = 0; i <= width; i++) {
    ctx.moveTo(i * (CELL_SIZE + 1) + 1, 0);
    ctx.lineTo(i * (CELL_SIZE + 1) + 1, (CELL_SIZE + 1) * height + 1);
  }

  // Horizontal lines.
  for (let j = 0; j <= height; j++) {
    ctx.moveTo(0,                           j * (CELL_SIZE + 1) + 1);
    ctx.lineTo((CELL_SIZE + 1) * width + 1, j * (CELL_SIZE + 1) + 1);
  }

  ctx.stroke();
};


const getIndex = (width: number, row: number, column: number) => {
  return row * width + column;
};

const drawCells = (
  ctx: CanvasRenderingContext2D,
  universe: Universe,
  width: number,
  height: number
) => {
  const cellsPtr = universe.cells();
  const cells = new Uint8Array(memory.buffer, cellsPtr, width * height);

  ctx.beginPath();

  for (let row = 0; row < height; row++) {
    for (let col = 0; col < width; col++) {
      const idx = getIndex(width, row, col);

      ctx.fillStyle = cells[idx] === Cell.Dead
        ? DEAD_COLOR
        : ALIVE_COLOR;

      ctx.fillRect(
        col * (CELL_SIZE + 1) + 1,
        row * (CELL_SIZE + 1) + 1,
        CELL_SIZE,
        CELL_SIZE
      );
    }
  }

  ctx.stroke();
};

export const initGame = (canvas: HTMLCanvasElement) => {
  init_webgl(canvas);
}

export const initGameOfLife = (canvas: HTMLCanvasElement) => {
  const universe = Universe.new(64, 64);
  const width = universe.width();
  const height = universe.height();


  const ctx = canvas.getContext("2d");
  if(!ctx) { throw new Error("missing canvas context"); }

  // resize canvas
  canvas.height = (CELL_SIZE + 1) * height + 1;
  canvas.width = (CELL_SIZE + 1) * width + 1;

  const render = () => {
    drawGrid(ctx, width, height);
    drawCells(ctx, universe, width, height);
  }

  const renderLoop = () => {
    universe.tick();

    render();

    requestAnimationFrame(renderLoop);
  }

  render();
  requestAnimationFrame(renderLoop);
}

export const foo = 5;