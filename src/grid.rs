// src/grid.rs

pub const COST_NORMAL: u16 = 1;
pub const COST_MUD: u16 = 3;
pub const COST_WATER: u16 = 5;

#[derive(Clone, Copy, Default)]
pub struct Cell {
    pub is_wall: bool,
    pub is_start: bool,
    pub is_end: bool,
    pub is_visited: bool,
    pub is_path: bool,
    pub cost: u16,
}

impl Cell {
    pub fn new() -> Self {
        Self { cost: COST_NORMAL, ..Default::default() }
    }
}

pub struct Grid {
    pub width: usize,
    pub height: usize,
    cells: Vec<Cell>,
}

impl Grid {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            cells: vec![Cell::new(); width * height],
        }
    }

    pub fn idx(&self, row: usize, col: usize) -> usize {
        row * self.width + col
    }

    pub fn cell(&self, idx: usize) -> &Cell {
        &self.cells[idx]
    }

    pub fn cell_mut(&mut self, idx: usize) -> &mut Cell {
        &mut self.cells[idx]
    }

    pub fn set_wall(&mut self, idx: usize, value: bool) {
        self.cells[idx].is_wall = value;
        self.cells[idx].cost = if value { 0 } else { COST_NORMAL };
    }

    pub fn set_start(&mut self, idx: usize) {
        for cell in self.cells.iter_mut() { cell.is_start = false; }
        self.cells[idx].is_start = true;
    }

    pub fn set_end(&mut self, idx: usize) {
        for cell in self.cells.iter_mut() { cell.is_end = false; }
        self.cells[idx].is_end = true;
    }

    pub fn set_terrain(&mut self, idx: usize, cost: u16) {
        if !self.cells[idx].is_wall {
            self.cells[idx].cost = cost;
        }
    }

    pub fn reset_search_state(&mut self) {
        for cell in self.cells.iter_mut() {
            cell.is_visited = false;
            cell.is_path = false;
        }
    }

    pub fn reset_all(&mut self) {
        for cell in self.cells.iter_mut() { *cell = Cell::new(); }
    }

    pub fn neighbors(&self, idx: usize) -> Vec<usize> {
        let row = idx / self.width;
        let col = idx % self.width;
        let mut result = Vec::with_capacity(4);

        if row > 0               { result.push(idx - self.width); }
        if row < self.height - 1 { result.push(idx + self.width); }
        if col > 0               { result.push(idx - 1); }
        if col < self.width - 1  { result.push(idx + 1); }

        result.into_iter()
            .filter(|&i| !self.cells[i].is_wall)
            .collect()
    }
}