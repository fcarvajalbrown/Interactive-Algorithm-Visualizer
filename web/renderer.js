// web/renderer.js

// The CELL_COLORS map is the only contract between JS and Rust
// if you ever add a cell type, you add a byte value in to_render_byte() 
// and a color here. That's the full rendering contract in one place. 

export class Renderer {
  constructor(canvas, wasm, lab, gridWidth, gridHeight) {
    this.canvas = canvas;
    this.ctx = canvas.getContext("2d");
    this.wasm = wasm;
    this.lab = lab;
    this.gridWidth = gridWidth;
    this.gridHeight = gridHeight;

    // Fit canvas to window on init and resize
    this.resize();
    window.addEventListener("resize", () => this.resize());
  }

  resize() {
    this.canvas.width = this.canvas.clientWidth;
    this.canvas.height = this.canvas.clientHeight;
    this.cellW = this.canvas.width / this.gridWidth;
    this.cellH = this.canvas.height / this.gridHeight;
    this.draw();
  }

  draw() {
    const ptr = this.lab.render_buffer_ptr();
    const len = this.lab.render_buffer_len();

    // Zero-copy read from WASM memory
    const buffer = new Uint8Array(this.wasm.memory.buffer, ptr, len);

    const ctx = this.ctx;
    const cw = this.cellW;
    const ch = this.cellH;

    for (let i = 0; i < len; i++) {
      const col = i % this.gridWidth;
      const row = Math.floor(i / this.gridWidth);
      ctx.fillStyle = CELL_COLORS[buffer[i]] ?? CELL_COLORS[0];
      ctx.fillRect(col * cw, row * ch, cw, ch);
    }

    // Draw grid lines only if cells are large enough to warrant it
    if (cw > 6) {
      ctx.strokeStyle = "rgba(0,0,0,0.08)";
      ctx.lineWidth = 0.5;
      for (let r = 0; r <= this.gridHeight; r++) {
        ctx.beginPath();
        ctx.moveTo(0, r * ch);
        ctx.lineTo(this.canvas.width, r * ch);
        ctx.stroke();
      }
      for (let c = 0; c <= this.gridWidth; c++) {
        ctx.beginPath();
        ctx.moveTo(c * cw, 0);
        ctx.lineTo(c * cw, this.canvas.height);
        ctx.stroke();
      }
    }
  }
}

// Must match to_render_byte() encoding in grid.rs
const CELL_COLORS = {
  0: "#1e1e2e", // empty
  1: "#45475a", // wall
  2: "#a6e3a1", // start (green)
  3: "#f38ba8", // end (red)
  4: "#89b4fa", // visited (blue)
  5: "#f9e2af", // path (yellow)
  6: "#a18a5a", // mud
  7: "#5a8aa1", // water
};