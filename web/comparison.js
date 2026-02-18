// web/comparison.js
export class Comparison {
  constructor(canvas, wasm, lab, gridWidth, gridHeight) {
    this.canvas = canvas;
    this.ctx = canvas.getContext("2d");
    this.wasm = wasm;
    this.lab = lab;
    this.gridWidth = gridWidth;
    this.gridHeight = gridHeight;
    this.active = false;
  }

  enable(algoA, algoB) {
    this.active = true;
    this.algoA = algoA; // e.g. "bfs"
    this.algoB = algoB; // e.g. "astar"
  }

  disable() {
    this.active = false;
  }

  run() {
    if (!this.active) return;

    // Run algo A, snapshot its render buffer
    this.lab.reset_search();
    const resultA = this.lab[`run_${this.algoA}`]();
    const ptrA = this.lab.render_buffer_ptr();
    const len = this.lab.render_buffer_len();
    const bufferA = new Uint8Array(this.wasm.memory.buffer, ptrA, len).slice(); // copy

    // Run algo B, read its render buffer
    this.lab.reset_search();
    const resultB = this.lab[`run_${this.algoB}`]();
    const ptrB = this.lab.render_buffer_ptr();
    const bufferB = new Uint8Array(this.wasm.memory.buffer, ptrB, len).slice();

    this._drawSplit(bufferA, bufferB);

    return { resultA, resultB };
  }

  _drawSplit(bufferA, bufferB) {
    const ctx = this.ctx;
    const halfW = this.canvas.width / 2;
    const cw = halfW / this.gridWidth;
    const ch = this.canvas.height / this.gridHeight;

    ctx.clearRect(0, 0, this.canvas.width, this.canvas.height);

    for (let i = 0; i < bufferA.length; i++) {
      const col = i % this.gridWidth;
      const row = Math.floor(i / this.gridWidth);

      // Left half — algo A
      ctx.fillStyle = CELL_COLORS[bufferA[i]] ?? CELL_COLORS[0];
      ctx.fillRect(col * cw, row * ch, cw, ch);

      // Right half — algo B
      ctx.fillStyle = CELL_COLORS[bufferB[i]] ?? CELL_COLORS[0];
      ctx.fillRect(halfW + col * cw, row * ch, cw, ch);
    }

    // Divider line
    ctx.strokeStyle = "#cdd6f4";
    ctx.lineWidth = 2;
    ctx.beginPath();
    ctx.moveTo(halfW, 0);
    ctx.lineTo(halfW, this.canvas.height);
    ctx.stroke();

    // Labels
    ctx.fillStyle = "#cdd6f4";
    ctx.font = "bold 14px monospace";
    ctx.fillText(this.algoA.toUpperCase(), 10, 20);
    ctx.fillText(this.algoB.toUpperCase(), halfW + 10, 20);
  }
}

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