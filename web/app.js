// web/app.js
console.log("app.js executing");
const GRID_WIDTH = 60;
const GRID_HEIGHT = 40;

const CELL_COLORS = {
  0: "#1e1e2e",
  1: "#45475a",
  2: "#a6e3a1",
  3: "#f38ba8",
  4: "#89b4fa",
  5: "#f9e2af",
  6: "#a18a5a",
  7: "#5a8aa1",
};

let lab = null;
let wasm = null;
let canvas = null;
let ctx = null;
let cellW = 0;
let cellH = 0;

async function startApp() {
  wasm = window.__wasm;
  lab = new window.__AlgoLab(GRID_WIDTH, GRID_HEIGHT);
  canvas = document.getElementById("grid-canvas");
  ctx = canvas.getContext("2d");

  // Set start/end BEFORE any draw call
  lab.set_start(gridIdx(1, 1));
  lab.set_end(gridIdx(GRID_HEIGHT - 2, GRID_WIDTH - 2));

  // Now safe to size and draw
lab.set_start(gridIdx(1, 1));
  lab.set_end(gridIdx(GRID_HEIGHT - 2, GRID_WIDTH - 2));

  setTimeout(() => {
    computeSize();
    draw();
  }, 100);

  window.addEventListener("resize", () => { computeSize(); draw(); });

  bindButtons();
  bindInput();
}

function gridIdx(row, col) {
  return row * GRID_WIDTH + col;
}

function computeSize() {
  canvas.width = canvas.clientWidth;
  canvas.height = canvas.clientHeight;
  cellW = canvas.width / GRID_WIDTH;
  cellH = canvas.height / GRID_HEIGHT;
}

function draw() {
  if (!lab || !wasm) return;
  const ptr = lab.render_buffer_ptr();
  const len = lab.render_buffer_len();
  const buffer = new Uint8Array(wasm.memory.buffer, ptr, len);

  for (let i = 0; i < len; i++) {
    const col = i % GRID_WIDTH;
    const row = Math.floor(i / GRID_WIDTH);
    ctx.fillStyle = CELL_COLORS[buffer[i]] ?? CELL_COLORS[0];
    ctx.fillRect(col * cellW, row * cellH, cellW, cellH);
  }

  if (cellW > 6) {
    ctx.strokeStyle = "rgba(0,0,0,0.08)";
    ctx.lineWidth = 0.5;
    for (let r = 0; r <= GRID_HEIGHT; r++) {
      ctx.beginPath();
      ctx.moveTo(0, r * cellH);
      ctx.lineTo(canvas.width, r * cellH);
      ctx.stroke();
    }
    for (let c = 0; c <= GRID_WIDTH; c++) {
      ctx.beginPath();
      ctx.moveTo(c * cellW, 0);
      ctx.lineTo(c * cellW, canvas.height);
      ctx.stroke();
    }
  }
}

function bindInput() {
  let isMouseDown = false;
  let drawMode = null;

  function cellFromEvent(e) {
    const rect = canvas.getBoundingClientRect();
    const clientX = e.touches ? e.touches[0].clientX : e.clientX;
    const clientY = e.touches ? e.touches[0].clientY : e.clientY;
    const col = Math.floor(((clientX - rect.left) / rect.width) * GRID_WIDTH);
    const row = Math.floor(((clientY - rect.top) / rect.height) * GRID_HEIGHT);
    if (col < 0 || col >= GRID_WIDTH || row < 0 || row >= GRID_HEIGHT) return null;
    return { row, col, idx: gridIdx(row, col) };
  }

  canvas.addEventListener("mousedown", (e) => {
    e.preventDefault();
    isMouseDown = true;
    const cell = cellFromEvent(e);
    if (!cell) return;
    drawMode = (e.button === 2 || e.shiftKey) ? "erase" : "wall";
    lab.set_wall(cell.idx, drawMode === "wall");
    draw();
  });

  canvas.addEventListener("mousemove", (e) => {
    e.preventDefault();
    if (!isMouseDown) return;
    const cell = cellFromEvent(e);
    if (!cell) return;
    lab.set_wall(cell.idx, drawMode === "wall");
    draw();
  });

  canvas.addEventListener("mouseup", () => { isMouseDown = false; drawMode = null; });
  canvas.addEventListener("contextmenu", (e) => e.preventDefault());

  canvas.addEventListener("touchstart", (e) => { e.preventDefault(); isMouseDown = true; const cell = cellFromEvent(e); if (!cell) return; lab.set_wall(cell.idx, true); draw(); }, { passive: false });
  canvas.addEventListener("touchmove", (e) => { e.preventDefault(); if (!isMouseDown) return; const cell = cellFromEvent(e); if (!cell) return; lab.set_wall(cell.idx, true); draw(); }, { passive: false });
  canvas.addEventListener("touchend", () => { isMouseDown = false; });

  document.addEventListener("keydown", (e) => {
    if (e.code === "KeyR") resetAll();
  });
}

function activateTerrainMode(cost) {
  function onTerrainClick(e) {
    const rect = canvas.getBoundingClientRect();
    const col = Math.floor(((e.clientX - rect.left) / rect.width) * GRID_WIDTH);
    const row = Math.floor(((e.clientY - rect.top) / rect.height) * GRID_HEIGHT);
    lab.set_terrain(gridIdx(row, col), cost);
    draw();
    canvas.removeEventListener("click", onTerrainClick);
  }
  canvas.addEventListener("click", onTerrainClick);
}

function runAlgo(name) {
  lab.reset_search();
  const result = lab[`run_${name}`]();
  draw();
  displayStats(result);
}

function resetAll() {
  lab.reset_all();
  lab.set_start(gridIdx(1, 1));
  lab.set_end(gridIdx(GRID_HEIGHT - 2, GRID_WIDTH - 2));
  draw();
  clearStats();
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

function bindButtons() {
  document.getElementById("btn-bfs").onclick = () => runAlgo("bfs");
  document.getElementById("btn-dfs").onclick = () => runAlgo("dfs");
  document.getElementById("btn-dijkstra").onclick = () => runAlgo("dijkstra");
  document.getElementById("btn-astar").onclick = () => runAlgo("astar");

  document.getElementById("btn-maze-backtracker").onclick = () => {
    lab.generate_maze_backtracker();
    draw();
  };
  document.getElementById("btn-maze-prims").onclick = () => {
    lab.generate_maze_prims();
    draw();
  };

  document.getElementById("btn-terrain-mud").onclick = () => activateTerrainMode(3);
  document.getElementById("btn-terrain-water").onclick = () => activateTerrainMode(5);

  document.getElementById("btn-reset-search").onclick = () => {
    lab.reset_search();
    draw();
    clearStats();
  };
  document.getElementById("btn-reset-all").onclick = resetAll;
}

window.startApp = startApp;