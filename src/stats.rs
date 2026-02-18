// src/stats.rs
use crate::AlgoResult;

#[derive(Default, Clone, Copy)]
pub struct Stats {
    pub nodes_explored: u32,
    pub path_length: u32,
    pub execution_ms: f64,
    pub path_found: bool,
}

impl Stats {
    pub fn to_result(&self) -> AlgoResult {
        AlgoResult {
            nodes_explored: self.nodes_explored,
            path_length: self.path_length,
            execution_ms: self.execution_ms,
            path_found: self.path_found,
        }
    }
}