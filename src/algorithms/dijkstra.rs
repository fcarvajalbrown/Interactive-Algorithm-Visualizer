// src/algorithms/dijkstra.rs
use std::collections::BinaryHeap;
use std::cmp::Reverse;
use crate::grid::Grid;
use crate::stats::Stats;
use super::reconstruct_path;

pub fn run(grid: &mut Grid) -> Stats {
    let start_time = macroquad::time::get_time();

    let mut start_idx = None;
    let mut end_idx = None;
    for i in 0..(grid.width * grid.height) {
        if grid.cell(i).is_start { start_idx = Some(i); }
        if grid.cell(i).is_end   { end_idx = Some(i); }
    }

    let (Some(start), Some(end)) = (start_idx, end_idx) else {
        return Stats::default();
    };

    let size = grid.width * grid.height;
    let mut dist = vec![u32::MAX; size];
    let mut parent = vec![usize::MAX; size];
    let mut nodes_explored: u32 = 0;
    let mut heap = BinaryHeap::new();

    dist[start] = 0;
    heap.push(Reverse((0u32, start)));

    'search: while let Some(Reverse((cost, current))) = heap.pop() {
        if cost > dist[current] { continue; }
        nodes_explored += 1;
        if current == end { break 'search; }
        let cell = grid.cell_mut(current);
        if !cell.is_start { cell.is_visited = true; }
        for neighbor in grid.neighbors(current) {
            let neighbor_cost = grid.cell(neighbor).cost as u32;
            let next_cost = cost + neighbor_cost;
            if next_cost < dist[neighbor] {
                dist[neighbor] = next_cost;
                parent[neighbor] = current;
                heap.push(Reverse((next_cost, neighbor)));
            }
        }
    }

    let path_length = reconstruct_path(grid, &parent, start, end);
    let execution_ms = (macroquad::time::get_time() - start_time) * 1000.0;
    Stats { nodes_explored, path_length, execution_ms, path_found: path_length > 0 }
}