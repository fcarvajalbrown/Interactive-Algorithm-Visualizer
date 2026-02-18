// src/maze/recursive_backtracker.rs
use crate::grid::Grid;

/// Generates a maze using the Recursive Backtracker (aka Depth-First Search) algorithm.
/// Starts with a fully walled grid and carves passages by visiting unvisited neighbors,
/// knocking down the wall between the current cell and the chosen neighbor.
pub fn generate(grid: &mut Grid) {
    // Fill everything with walls first
    for i in 0..(grid.width * grid.height) {
        if let Some(cell) = grid.get_mut(i) {
            cell.is_wall = true;
            cell.cost = 0;
        }
    }

    let mut visited = vec![false; grid.width * grid.height];
    let mut stack = Vec::new();

    // Start from top-left cell (must be odd coords to keep wall structure clean)
    let start = grid.idx(1, 1);
    visited[start] = true;
    carve(grid, start);
    stack.push(start);

    while let Some(&current) = stack.last() {
        let unvisited = maze_neighbors(grid, current)
            .into_iter()
            .filter(|&(neighbor, _)| !visited[neighbor])
            .collect::<Vec<_>>();

        if unvisited.is_empty() {
            stack.pop();
        } else {
            // Pick a random unvisited neighbor
            let (neighbor, wall_between) = unvisited[random_index(unvisited.len())];

            // Carve through the wall between current and neighbor
            carve(grid, wall_between);
            carve(grid, neighbor);

            visited[neighbor] = true;
            stack.push(neighbor);
        }
    }
}

/// Removes the wall at a given index (makes it a passable empty cell).
fn carve(grid: &mut Grid, idx: usize) {
    if let Some(cell) = grid.get_mut(idx) {
        cell.is_wall = false;
        cell.cost = 1;
    }
}

/// Returns maze-valid neighbors: cells 2 steps away (skipping over the wall between).
/// Returns (neighbor_idx, wall_idx) pairs.
fn maze_neighbors(grid: &Grid, idx: usize) -> Vec<(usize, usize)> {
    let row = idx / grid.width;
    let col = idx % grid.width;
    let mut result = Vec::new();

    // Up
    if row >= 2 {
        result.push((grid.idx(row - 2, col), grid.idx(row - 1, col)));
    }
    // Down
    if row + 2 < grid.height {
        result.push((grid.idx(row + 2, col), grid.idx(row + 1, col)));
    }
    // Left
    if col >= 2 {
        result.push((grid.idx(row, col - 2), grid.idx(row, col - 1)));
    }
    // Right
    if col + 2 < grid.width {
        result.push((grid.idx(row, col + 2), grid.idx(row, col + 1)));
    }

    result
}

/// Minimal WASM-compatible random index using js_sys.
fn random_index(len: usize) -> usize {
    (js_sys::Math::random() * len as f64) as usize
}