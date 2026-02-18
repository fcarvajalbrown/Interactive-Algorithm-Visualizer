// web/main.js
import init, { AlgoLab } from "/rust-wasm-algo-lab.js";
import { Renderer } from "./renderer.js";
import { bindInput } from "/web/input.js";

const GRID_WIDTH = 60;
const GRID_HEIGHT = 40;

async function main() {
  // Initialize WASM module
  const wasm = await init();

  const lab = new AlgoLab(GRID_WIDTH, GRID_HEIGHT);
  const canvas = document.getElementById("grid-canvas");

  const renderer = new Renderer(canvas, wasm, lab, GRID_WIDTH, GRID_HEIGHT);
  bindInput(canvas, lab, renderer, GRID_WIDTH, GRID_HEIGHT);

  // Set default start and end points
  lab.set_start(grid_idx(1, 1, GRID_WIDTH));
  lab.set_end(grid_idx(GRID_HEIGHT - 2, GRID_WIDTH - 2, GRID_WIDTH));
  renderer.draw();

  // Wire up control buttons
  document.getElementById("btn-bfs").onclick = () => runAlgo("bfs", lab, renderer);
  document.getElementById("btn-dfs").onclick = () => runAlgo("dfs", lab, renderer);
  document.getElementById("btn-dijkstra").onclick = () => runAlgo("dijkstra", lab, renderer);
  document.getElementById("btn-astar").onclick = () => runAlgo("astar", lab, renderer);

  document.getElementById("btn-maze-backtracker").onclick = () => {
    lab.generate_maze_backtracker();
    renderer.draw();
  };
  document.getElementById("btn-maze-prims").onclick = () => {
    lab.generate_maze_prims();
    renderer.draw();
  };

  document.getElementById("btn-reset-search").onclick = () => {
    lab.reset_search();
    renderer.draw();
    clearStats();
  };
  document.getElementById("btn-reset-all").onclick = () => {
    lab.reset_all();
    lab.set_start(grid_idx(1, 1, GRID_WIDTH));
    lab.set_end(grid_idx(GRID_HEIGHT - 2, GRID_WIDTH - 2, GRID_WIDTH));
    renderer.draw();
    clearStats();
  };

  // Keyboard shortcuts
  document.addEventListener("keydown", (e) => {
    if (e.code === "KeyR") document.getElementById("btn-reset-all").click();
  });
}

function runAlgo(name, lab, renderer) {
  lab.reset_search();
  const result = lab[`run_${name}`]();
  renderer.draw();
  displayStats(result);
}

function displayStats(result) {
  document.getElementById("stat-nodes").textContent = result.nodes_explored();
  document.getElementById("stat-path").textContent = result.path_found()
    ? result.path_length()
    : "No path";
  document.getElementById("stat-time").textContent = result.execution_ms().toFixed(2) + " ms";
}

function clearStats() {
  document.getElementById("stat-nodes").textContent = "-";
  document.getElementById("stat-path").textContent = "-";
  document.getElementById("stat-time").textContent = "-";
}

export function grid_idx(row, col, width) {
  return row * width + col;
}

main();