// src/stats.rs

#[derive(Default, Clone, Copy)]
pub struct Stats {
    pub nodes_explored: u32,
    pub path_length: u32,
    pub execution_ms: f64,
    pub path_found: bool,
}