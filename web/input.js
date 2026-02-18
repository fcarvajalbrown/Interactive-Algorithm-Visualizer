// web/input.js
import { grid_idx } from "/web/main.js";

export function bindInput(canvas, lab, renderer, gridWidth, gridHeight) {
  let isMouseDown = false;
  let drawMode = null;

  function cellFromEvent(e) {
    const rect = canvas.getBoundingClientRect();
    const clientX = e.touches ? e.touches[0].clientX : e.clientX;
    const clientY = e.touches ? e.touches[0].clientY : e.clientY;
    const col = Math.floor(((clientX - rect.left) / rect.width) * gridWidth);
    const row = Math.floor(((clientY - rect.top) / rect.height) * gridHeight);
    if (col < 0 || col >= gridWidth || row < 0 || row >= gridHeight) return null;
    return { row, col, idx: grid_idx(row, col, gridWidth) };
  }

  function handleDown(e) {
    e.preventDefault();
    isMouseDown = true;
    const cell = cellFromEvent(e);
    if (!cell) return;

    if (e.button === 2 || e.shiftKey) {
      drawMode = "erase";
      lab.set_wall(cell.idx, false);
    } else {
      drawMode = "wall";
      lab.set_wall(cell.idx, true);
    }
    renderer.draw();
  }

  function handleMove(e) {
    e.preventDefault();
    if (!isMouseDown) return;
    const cell = cellFromEvent(e);
    if (!cell) return;
    lab.set_wall(cell.idx, drawMode === "wall");
    renderer.draw();
  }

  function handleUp(e) {
    isMouseDown = false;
    drawMode = null;
  }

  canvas.addEventListener("mousedown", handleDown);
  canvas.addEventListener("mousemove", handleMove);
  canvas.addEventListener("mouseup", handleUp);
  canvas.addEventListener("contextmenu", (e) => e.preventDefault());

  canvas.addEventListener("touchstart", handleDown, { passive: false });
  canvas.addEventListener("touchmove", handleMove, { passive: false });
  canvas.addEventListener("touchend", handleUp);

  document.getElementById("btn-terrain-mud")?.addEventListener("click", () => {
    setTerrainMode(canvas, lab, renderer, gridWidth, gridHeight, 3);
  });
  document.getElementById("btn-terrain-water")?.addEventListener("click", () => {
    setTerrainMode(canvas, lab, renderer, gridWidth, gridHeight, 5);
  });
}

function setTerrainMode(canvas, lab, renderer, gridWidth, gridHeight, cost) {
  function onTerrainClick(e) {
    const rect = canvas.getBoundingClientRect();
    const col = Math.floor(((e.clientX - rect.left) / rect.width) * gridWidth);
    const row = Math.floor(((e.clientY - rect.top) / rect.height) * gridHeight);
    const idx = grid_idx(row, col, gridWidth);
    console.log("terrain click", { row, col, idx, cost });
    lab.set_terrain(idx, cost);
    renderer.draw();
    canvas.removeEventListener("click", onTerrainClick);
  }
  canvas.addEventListener("click", onTerrainClick);
}