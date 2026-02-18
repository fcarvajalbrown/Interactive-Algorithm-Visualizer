// src/algorithms/astar.rs
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
    let mut g_cost = vec![u32::MAX; size]; // actual cost from start
    let mut parent = vec![usize::MAX; size];
    let mut nodes_explored: u32 = 0;

    // f = g + h, heap ordered by f
    let mut heap = BinaryHeap::new();

    g_cost[start] = 0;
    let h = manhattan(start, end, grid.width);
    heap.push(Reverse((h, start)));

    'search: while let Some(Reverse((_f, current))) = heap.pop() {
        // Stale entry check
        if current != start {
            let current_g = g_cost[current];
            let expected_f = current_g + manhattan(current, end, grid.width);
            if _f > expected_f { continue; }
        }

        nodes_explored += 1;

        if current == end { break 'search; }

        if let Some(cell) = grid.get_mut(current) {
            if !cell.is_start { cell.is_visited = true; }
        }

        for neighbor in grid.neighbors(current) {
            let neighbor_cost = grid.get(neighbor)
                .map(|c| c.cost as u32)
                .unwrap_or(1);

            let tentative_g = g_cost[current] + neighbor_cost;

            if tentative_g < g_cost[neighbor] {
                g_cost[neighbor] = tentative_g;
                parent[neighbor] = current;
                let f = tentative_g + manhattan(neighbor, end, grid.width);
                heap.push(Reverse((f, neighbor)));
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

/// Manhattan distance heuristic — admissible for 4-directional grids.
/// Never overestimates, so A* remains optimal.
fn manhattan(idx: usize, end: usize, width: usize) -> u32 {
    let (r1, c1) = (idx / width, idx % width);
    let (r2, c2) = (end / width, end % width);
    ((r1 as i32 - r2 as i32).unsigned_abs()
        + (c1 as i32 - c2 as i32).unsigned_abs())
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