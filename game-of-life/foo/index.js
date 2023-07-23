import { memory } from "../pkg/game_of_life_bg";
import { Cell, Universe } from "../pkg/game_of_life";

// Define color and size constants
const CELL_SIZE = 10;
const GRID_COLOR = "#CCCCCC";
const DEAD_COLOR = "#FFFFFF";
const ALIVE_COLOR = "#000000";

// Create a new universe
const universe = Universe.new();
const width = universe.width();
const height = universe.height();

// Setup the canvas
const canvas = document.getElementById("game-of-life-canvas");
canvas.height = (CELL_SIZE + 1) * height + 1;
canvas.width = (CELL_SIZE + 1) * width + 1;
const ctx = canvas.getContext("2d");

// Calculate position based on click event
const calculatePosition = (event) => {
  const boundingRect = canvas.getBoundingClientRect();
  const scaleX = canvas.width / boundingRect.width;
  const scaleY = canvas.height / boundingRect.height;
  const canvasLeft = (event.clientX - boundingRect.left) * scaleX;
  const canvasTop = (event.clientY - boundingRect.top) * scaleY;

  return {
    row: Math.min(Math.floor(canvasTop / (CELL_SIZE + 1)), height - 1),
    col: Math.min(Math.floor(canvasLeft / (CELL_SIZE + 1)), width - 1),
  };
};

// Handle cell toggle on click
canvas.addEventListener("click", (event) => {
  const { row, col } = calculatePosition(event);
  universe.toggle_cell(row, col);
  drawGrid();
  drawCells();
});

// Animation control
let animationId = null;
const renderLoop = () => {
  universe.tick();
  drawGrid();
  drawCells();
  animationId = requestAnimationFrame(renderLoop);
};

// Control button for play and pause
const playPauseButton = document.getElementById("play-pause");
const play = () => {
  playPauseButton.textContent = "⏸";
  renderLoop();
};
const pause = () => {
  playPauseButton.textContent = "▶";
  cancelAnimationFrame(animationId);
  animationId = null;
};
const isPaused = () => animationId === null;

playPauseButton.addEventListener("click", () => {
  isPaused() ? play() : pause();
});

// Draw the grid
const drawGrid = () => {
  ctx.beginPath();
  ctx.strokeStyle = GRID_COLOR;

  // Draw vertical lines
  for (let index = 0; index <= width; index++) {
    ctx.moveTo(index * (CELL_SIZE + 1) + 1, 0);
    ctx.lineTo(index * (CELL_SIZE + 1) + 1, (CELL_SIZE + 1) * height + 1);
  }

  // Draw horizontal lines
  for (let index = 0; index <= height; index++) {
    ctx.moveTo(0, index * (CELL_SIZE + 1) + 1);
    ctx.lineTo((CELL_SIZE + 1) * width + 1, index * (CELL_SIZE + 1) + 1);
  }

  ctx.stroke();
};

// Define helper function to calculate index
const getIndex = (row, column) => row * height + column;

// Draw the living and dead cells
const drawCells = () => {
  const cellsPtr = universe.cells();
  const cells = new Uint8Array(memory.buffer, cellsPtr, width * height);
  ctx.beginPath();

  for (let row = 0; row < height; row++) {
    for (let col = 0; col < width; col++) {
      const index = getIndex(row, col);
      ctx.fillStyle = cells[index] === Cell.Dead ? DEAD_COLOR : ALIVE_COLOR;
      ctx.fillRect(
        col * (CELL_SIZE + 1) + 1,
        row * (CELL_SIZE + 1) + 1,
        CELL_SIZE,
        CELL_SIZE,
      );
    }
  }

  ctx.stroke();
};

// Initial rendering
drawGrid();
drawCells();
play();
