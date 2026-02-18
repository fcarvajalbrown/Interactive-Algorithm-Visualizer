// src/grid.rs
use wasm_bindgen::prelude::*;

/// Traversal cost constants
pub const COST_NORMAL: u16 = 1;
pub const COST_MUD: u16 = 3;
pub const COST_WATER: u16 = 5;

/// A single cell on the grid.
/// Using a struct with flags keeps it flexible for weighted terrain and state tracking.
#[derive(Clone, Copy, Default)]
pub struct Cell {
    pub is_wall: bool,
    pub is_start: bool,
    pub is_end: bool,
    pub is_visited: bool,
    pub is_path: bool,
    pub cost: u16, // traversal cost (1 = normal, 3 = mud, 5 = water)
}

impl Cell {
    pub fn new() -> Self {
        Self {
            cost: COST_NORMAL,
            ..Default::default()
        }
    }

    pub fn wall() -> Self {
        Self { is_wall: true, cost: 0, ..Default::default() }
    }

    /// Encodes cell state as a u8 for the shared memory buffer.
    /// JS reads this byte to decide what color to paint each cell.
    ///
    /// Encoding:
    ///   0 = empty
    ///   1 = wall
    ///   2 = start
    ///   3 = end
    ///   4 = visited
    ///   5 = path
    ///   6 = mud
    ///   7 = water
    pub fn to_render_byte(&self) -> u8 {
        if self.is_wall    { return 1; }
        if self.is_start   { return 2; }
        if self.is_end     { return 3; }
        if self.is_path    { return 5; }
        if self.is_visited { return 4; }
        match self.cost {
            COST_MUD   => 6,
            COST_WATER => 7,
            _          => 0,
        }
    }
}

/// The main grid structure. Owns all cell state.
/// Exposes a flat u8 render buffer that JS reads directly from WASM memory.
#[wasm_bindgen]
pub struct Grid {
    pub width: usize,
    pub height: usize,
    cells: Vec<Cell>,
    render_buffer: Vec<u8>, // flat array, length = width * height
}

#[wasm_bindgen]
impl Grid {
    #[wasm_bindgen(constructor)]
    pub fn new(width: usize, height: usize) -> Self {
        let size = width * height;
        Self {
            width,
            height,
            cells: vec![Cell::new(); size],
            render_buffer: vec![0u8; size],
        }
    }

    /// Returns a pointer to the render buffer so JS can read it
    /// via a Uint8Array view over WASM memory — zero copy.
    pub fn render_buffer_ptr(&self) -> *const u8 {
        self.render_buffer.as_ptr()
    }

    pub fn render_buffer_len(&self) -> usize {
        self.render_buffer.len()
    }

    /// Syncs cell state into the render buffer. Call this after any mutation.
    pub fn flush_render_buffer(&mut self) {
        for (i, cell) in self.cells.iter().enumerate() {
            self.render_buffer[i] = cell.to_render_byte();
        }
    }

    // --- Grid mutations (called from JS on user interaction) ---

    pub fn set_wall(&mut self, idx: usize, value: bool) {
        if let Some(cell) = self.cells.get_mut(idx) {
            cell.is_wall = value;
            cell.cost = if value { 0 } else { COST_NORMAL };
        }
    }

    pub fn set_start(&mut self, idx: usize) {
        // Clear previous start
        for cell in self.cells.iter_mut() { cell.is_start = false; }
        if let Some(cell) = self.cells.get_mut(idx) { cell.is_start = true; }
    }

    pub fn set_end(&mut self, idx: usize) {
        for cell in self.cells.iter_mut() { cell.is_end = false; }
        if let Some(cell) = self.cells.get_mut(idx) { cell.is_end = true; }
    }

    pub fn set_terrain(&mut self, idx: usize, cost: u16) {
        if let Some(cell) = self.cells.get_mut(idx) {
            if !cell.is_wall { cell.cost = cost; }
        }
    }

    pub fn reset_search_state(&mut self) {
        for cell in self.cells.iter_mut() {
            cell.is_visited = false;
            cell.is_path = false;
        }
    }

    pub fn reset_all(&mut self) {
        for cell in self.cells.iter_mut() {
            *cell = Cell::new();
        }
    }

    // --- Accessors for algorithms (internal use) ---

    pub fn idx(&self, row: usize, col: usize) -> usize {
        row * self.width + col
    }

    pub fn get(&self, idx: usize) -> Option<&Cell> {
        self.cells.get(idx)
    }

    pub fn get_mut(&mut self, idx: usize) -> Option<&mut Cell> {
        self.cells.get_mut(idx)
    }

    pub fn neighbors(&self, idx: usize) -> Vec<usize> {
        let row = idx / self.width;
        let col = idx % self.width;
        let mut result = Vec::with_capacity(4);

        if row > 0                  { result.push(idx - self.width); } // up
        if row < self.height - 1   { result.push(idx + self.width); } // down
        if col > 0                  { result.push(idx - 1); }          // left
        if col < self.width - 1    { result.push(idx + 1); }           // right

        result.into_iter()
            .filter(|&i| !self.cells[i].is_wall)
            .collect()
    }
}