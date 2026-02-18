// src/algorithms/mod.rs
pub mod bfs;
pub mod dfs;
pub mod dijkstra;
pub mod astar;

use crate::grid::Grid;

pub fn reconstruct_path(grid: &mut Grid, parent: &[usize], start: usize, end: usize) -> u32 {
    let mut length = 0;
    let mut current = end;

    if parent[end] == usize::MAX && end != start {
        return 0;
    }

    while current != start {
        let cell = grid.cell_mut(current);
        if !cell.is_end { cell.is_path = true; }
        current = parent[current];
        length += 1;
        if current == usize::MAX { return 0; }
    }

    length
}