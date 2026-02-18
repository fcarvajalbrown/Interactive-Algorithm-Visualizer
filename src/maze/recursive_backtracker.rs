// src/maze/recursive_backtracker.rs
use crate::grid::Grid;

pub fn generate(grid: &mut Grid) {
    for i in 0..(grid.width * grid.height) {
        let cell = grid.cell_mut(i);
        cell.is_wall = true;
        cell.cost = 0;
    }

    let mut visited = vec![false; grid.width * grid.height];
    let mut stack = Vec::new();

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
            let (neighbor, wall_between) = unvisited[random_index(unvisited.len())];
            carve(grid, wall_between);
            carve(grid, neighbor);
            visited[neighbor] = true;
            stack.push(neighbor);
        }
    }
}

fn carve(grid: &mut Grid, idx: usize) {
    let cell = grid.cell_mut(idx);
    cell.is_wall = false;
    cell.cost = 1;
}

fn maze_neighbors(grid: &Grid, idx: usize) -> Vec<(usize, usize)> {
    let row = idx / grid.width;
    let col = idx % grid.width;
    let mut result = Vec::new();

    if row >= 2          { result.push((grid.idx(row - 2, col), grid.idx(row - 1, col))); }
    if row + 2 < grid.height { result.push((grid.idx(row + 2, col), grid.idx(row + 1, col))); }
    if col >= 2          { result.push((grid.idx(row, col - 2), grid.idx(row, col - 1))); }
    if col + 2 < grid.width  { result.push((grid.idx(row, col + 2), grid.idx(row, col + 1))); }

    result
}

fn random_index(len: usize) -> usize {
    (js_sys::Math::random() * len as f64) as usize
}