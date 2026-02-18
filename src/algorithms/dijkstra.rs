// src/algorithms/dijkstra.rs
use std::collections::BinaryHeap;
use std::cmp::Reverse;
use web_sys::Performance;
use crate::grid::Grid;
use crate::stats::Stats;

pub fn run(grid: &mut Grid) -> Stats {
    let perf = web_sys::window()
        .and_then(|w| w.performance())
        .expect("performance API unavailable");

    let start_time = perf.now();

    let mut start_idx = None;
    let mut end_idx = None;
    for i in 0..(grid.width * grid.height) {
        if let Some(cell) = grid.get(i) {
            if cell.is_start { start_idx = Some(i); }
            if cell.is_end   { end_idx = Some(i); }
        }
    }

    let (Some(start), Some(end)) = (start_idx, end_idx) else {
        return Stats::default();
    };

    let size = grid.width * grid.height;
    let mut dist = vec![u32::MAX; size];
    let mut parent = vec![usize::MAX; size];
    let mut nodes_explored: u32 = 0;

    // BinaryHeap is a max-heap; Reverse flips it into a min-heap
    // Each entry: (Reverse(cost), cell_index)
    let mut heap = BinaryHeap::new();

    dist[start] = 0;
    heap.push(Reverse((0u32, start)));

    'search: while let Some(Reverse((cost, current))) = heap.pop() {
        // Skip stale entries (a cheaper path was already found)
        if cost > dist[current] { continue; }

        nodes_explored += 1;

        if current == end { break 'search; }

        if let Some(cell) = grid.get_mut(current) {
            if !cell.is_start { cell.is_visited = true; }
        }

        for neighbor in grid.neighbors(current) {
            let neighbor_cost = grid.get(neighbor)
                .map(|c| c.cost as u32)
                .unwrap_or(1);

            let next_cost = cost + neighbor_cost;

            if next_cost < dist[neighbor] {
                dist[neighbor] = next_cost;
                parent[neighbor] = current;
                heap.push(Reverse((next_cost, neighbor)));
            }
        }
    }

    let path_length = reconstruct_path(grid, &parent, start, end);
    let execution_ms = perf.now() - start_time;

    Stats {
        nodes_explored,
        path_length,
        execution_ms,
        path_found: path_length > 0,
    }
}

fn reconstruct_path(grid: &mut Grid, parent: &[usize], start: usize, end: usize) -> u32 {
    let mut length = 0;
    let mut current = end;

    if parent[end] == usize::MAX && end != start {
        return 0;
    }

    while current != start {
        if let Some(cell) = grid.get_mut(current) {
            if !cell.is_end { cell.is_path = true; }
        }
        current = parent[current];
        length += 1;
        if current == usize::MAX { return 0; }
    }

    length
}