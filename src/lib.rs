// src/lib.rs
use wasm_bindgen::prelude::*;

mod grid;
mod algorithms;
mod maze;
mod stats;

use grid::Grid;
use stats::Stats;

/// Top-level WASM entry point.
/// AlgoLab owns the Grid and Stats, and orchestrates algorithm execution.
/// JS only talks to this struct — it never directly instantiates Grid or Stats.
#[wasm_bindgen]
pub struct AlgoLab {
    grid: Grid,
    stats: Stats,
}

#[wasm_bindgen]
impl AlgoLab {
    #[wasm_bindgen(constructor)]
    pub fn new(width: usize, height: usize) -> Self {
        // Routes panic messages to browser console in debug builds
        #[cfg(feature = "console_error_panic_hook")]
        console_error_panic_hook::set_once();

        Self {
            grid: Grid::new(width, height),
            stats: Stats::default(),
        }
    }

    // --- Render buffer bridge (JS reads this every animation frame) ---

    pub fn render_buffer_ptr(&self) -> *const u8 {
        self.grid.render_buffer_ptr()
    }

    pub fn render_buffer_len(&self) -> usize {
        self.grid.render_buffer_len()
    }

    // --- Grid mutations (wired to mouse/touch events in input.js) ---

    pub fn set_wall(&mut self, idx: usize, active: bool) {
        self.grid.set_wall(idx, active);
        self.grid.flush_render_buffer();
    }

    pub fn set_start(&mut self, idx: usize) {
        self.grid.set_start(idx);
        self.grid.flush_render_buffer();
    }

    pub fn set_end(&mut self, idx: usize) {
        self.grid.set_end(idx);
        self.grid.flush_render_buffer();
    }

    pub fn set_terrain(&mut self, idx: usize, cost: u16) {
        web_sys::console::log_1(&format!("set_terrain idx={} cost={}", idx, cost).into());
        self.grid.set_terrain(idx, cost);
        self.grid.flush_render_buffer();
    }

    pub fn reset_search(&mut self) {
        self.grid.reset_search_state();
        self.stats = Stats::default();
        self.grid.flush_render_buffer();
    }

    pub fn reset_all(&mut self) {
        self.grid.reset_all();
        self.stats = Stats::default();
        self.grid.flush_render_buffer();
    }

    // --- Algorithm execution ---
    // Each returns a JS-friendly AlgoResult with stats.
    // The render buffer is updated incrementally inside each algorithm
    // so JS can call render_buffer_ptr() each frame to paint progress.

    pub fn run_bfs(&mut self) -> AlgoResult {
        self.grid.reset_search_state();
        self.stats = algorithms::bfs::run(&mut self.grid);
        self.grid.flush_render_buffer();
        self.stats.to_result()
    }

    pub fn run_dfs(&mut self) -> AlgoResult {
        self.grid.reset_search_state();
        self.stats = algorithms::dfs::run(&mut self.grid);
        self.grid.flush_render_buffer();
        self.stats.to_result()
    }

    pub fn run_dijkstra(&mut self) -> AlgoResult {
        self.grid.reset_search_state();
        self.stats = algorithms::dijkstra::run(&mut self.grid);
        self.grid.flush_render_buffer();
        self.stats.to_result()
    }

    pub fn run_astar(&mut self) -> AlgoResult {
        self.grid.reset_search_state();
        self.stats = algorithms::astar::run(&mut self.grid);
        self.grid.flush_render_buffer();
        self.stats.to_result()
    }

    // --- Maze generation ---

    pub fn generate_maze_backtracker(&mut self) {
        self.grid.reset_all();
        maze::recursive_backtracker::generate(&mut self.grid);
        self.grid.flush_render_buffer();
    }

    pub fn generate_maze_prims(&mut self) {
        self.grid.reset_all();
        maze::prims::generate(&mut self.grid);
        self.grid.flush_render_buffer();
    }
}

/// Returned to JS after each algorithm run.
/// wasm_bindgen can't return structs with complex fields directly,
/// so we use a flat getter-based struct instead.
#[wasm_bindgen]
pub struct AlgoResult {
    nodes_explored: u32,
    path_length: u32,
    execution_ms: f64,
    path_found: bool,
}

#[wasm_bindgen]
impl AlgoResult {
    pub fn nodes_explored(&self) -> u32 { self.nodes_explored }
    pub fn path_length(&self) -> u32 { self.path_length }
    pub fn execution_ms(&self) -> f64 { self.execution_ms }
    pub fn path_found(&self) -> bool { self.path_found }
}