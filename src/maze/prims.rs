// src/maze/prims.rs
use crate::grid::Grid;

pub fn generate(grid: &mut Grid) {
    for i in 0..(grid.width * grid.height) {
        let cell = grid.cell_mut(i);
        cell.is_wall = true;
        cell.cost = 0;
    }

    let mut in_maze = vec![false; grid.width * grid.height];
    let mut frontier: Vec<(usize, usize)> = Vec::new();

    let start = grid.idx(1, 1);
    carve(grid, start);
    in_maze[start] = true;

    for (neighbor, wall) in maze_neighbors(grid, start) {
        if !in_maze[neighbor] {
            frontier.push((wall, neighbor));
        }
    }

    while !frontier.is_empty() {
        let pick = random_index(frontier.len());
        let (wall, neighbor) = frontier.remove(pick);

        if in_maze[neighbor] { continue; }

        carve(grid, wall);
        carve(grid, neighbor);
        in_maze[neighbor] = true;

        for (next_neighbor, next_wall) in maze_neighbors(grid, neighbor) {
            if !in_maze[next_neighbor] {
                frontier.push((next_wall, next_neighbor));
            }
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
    macroquad::rand::gen_range(0, len)
}