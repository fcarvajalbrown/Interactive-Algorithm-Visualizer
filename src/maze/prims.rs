// src/maze/prims.rs
use crate::grid::Grid;

/// Generates a maze using Prim's algorithm.
/// Instead of following a single path like the backtracker,
/// it maintains a frontier of all reachable walls and picks one randomly.
/// This produces mazes with more branching and shorter dead ends.
pub fn generate(grid: &mut Grid) {
    // Fill everything with walls first
    for i in 0..(grid.width * grid.height) {
        if let Some(cell) = grid.get_mut(i) {
            cell.is_wall = true;
            cell.cost = 0;
        }
    }

    let mut in_maze = vec![false; grid.width * grid.height];
    let mut frontier: Vec<(usize, usize)> = Vec::new(); // (wall_idx, neighbor_idx)

    // Start from top-left
    let start = grid.idx(1, 1);
    carve(grid, start);
    in_maze[start] = true;

    // Add initial frontier
    for (neighbor, wall) in maze_neighbors(grid, start) {
        if !in_maze[neighbor] {
            frontier.push((wall, neighbor));
        }
    }

    while !frontier.is_empty() {
        // Pick a random frontier wall
        let pick = random_index(frontier.len());
        let (wall, neighbor) = frontier.remove(pick);

        if in_maze[neighbor] { continue; }

        // Carve through the wall and into the neighbor
        carve(grid, wall);
        carve(grid, neighbor);
        in_maze[neighbor] = true;

        // Add neighbor's unvisited neighbors to frontier
        for (next_neighbor, next_wall) in maze_neighbors(grid, neighbor) {
            if !in_maze[next_neighbor] {
                frontier.push((next_wall, next_neighbor));
            }
        }
    }
}

fn carve(grid: &mut Grid, idx: usize) {
    if let Some(cell) = grid.get_mut(idx) {
        cell.is_wall = false;
        cell.cost = 1;
    }
}

fn maze_neighbors(grid: &Grid, idx: usize) -> Vec<(usize, usize)> {
    let row = idx / grid.width;
    let col = idx % grid.width;
    let mut result = Vec::new();

    if row >= 2 {
        result.push((grid.idx(row - 2, col), grid.idx(row - 1, col)));
    }
    if row + 2 < grid.height {
        result.push((grid.idx(row + 2, col), grid.idx(row + 1, col)));
    }
    if col >= 2 {
        result.push((grid.idx(row, col - 2), grid.idx(row, col - 1)));
    }
    if col + 2 < grid.width {
        result.push((grid.idx(row, col + 2), grid.idx(row, col + 1)));
    }

    result
}

fn random_index(len: usize) -> usize {
    (js_sys::Math::random() * len as f64) as usize
}