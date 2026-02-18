// src/algorithms/dfs.rs
use crate::grid::Grid;
use crate::stats::Stats;
use super::reconstruct_path;

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
    let mut visited = vec![false; size];
    let mut parent = vec![usize::MAX; size];
    let mut stack = Vec::new();
    let mut nodes_explored: u32 = 0;

    stack.push(start);

    'search: while let Some(current) = stack.pop() {
        if visited[current] { continue; }
        visited[current] = true;
        nodes_explored += 1;

        if current == end { break 'search; }

        if let Some(cell) = grid.get_mut(current) {
            if !cell.is_start { cell.is_visited = true; }
        }

        for neighbor in grid.neighbors(current) {
            if !visited[neighbor] {
                parent[neighbor] = current;
                stack.push(neighbor);
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