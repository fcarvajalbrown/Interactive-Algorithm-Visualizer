// src/main.rs
use macroquad::prelude::*;

mod grid;
mod stats;
mod algorithms;
mod maze;
mod ui;

use grid::{Grid, COST_MUD, COST_WATER};
use stats::Stats;
use ui::Ui;

const GRID_WIDTH: usize = 60;
const GRID_HEIGHT: usize = 40;
const SIDEBAR_WIDTH: f32 = 200.0;

#[derive(PartialEq, Clone, Copy)]
pub enum Tool {
    Wall,
    Erase,
    Mud,
    Water,
}

pub struct AppState {
    pub grid: Grid,
    pub stats: Stats,
    pub last_algo: Option<&'static str>,
    pub active_tool: Tool,
}

impl AppState {
    pub fn new() -> Self {
        let mut grid = Grid::new(GRID_WIDTH, GRID_HEIGHT);
        grid.set_start(grid.idx(1, 1));
        grid.set_end(grid.idx(GRID_HEIGHT - 2, GRID_WIDTH - 2));
        Self {
            grid,
            stats: Stats::default(),
            last_algo: None,
            active_tool: Tool::Wall,
        }
    }

    pub fn run_algo(&mut self, name: &'static str) {
        self.grid.reset_search_state();
        self.stats = match name {
            "BFS"      => algorithms::bfs::run(&mut self.grid),
            "DFS"      => algorithms::dfs::run(&mut self.grid),
            "Dijkstra" => algorithms::dijkstra::run(&mut self.grid),
            "A*"       => algorithms::astar::run(&mut self.grid),
            _          => Stats::default(),
        };
        self.last_algo = Some(name);
    }

    pub fn reset_search(&mut self) {
        self.grid.reset_search_state();
        self.stats = Stats::default();
    }

    pub fn reset_all(&mut self) {
        self.grid.reset_all();
        self.grid.set_start(self.grid.idx(1, 1));
        self.grid.set_end(self.grid.idx(GRID_HEIGHT - 2, GRID_WIDTH - 2));
        self.stats = Stats::default();
        self.last_algo = None;
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Algorithm Lab".to_string(),
        window_width: 1280,
        window_height: 720,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut state = AppState::new();
    let mut ui = Ui::new();

    loop {
        let screen_w = screen_width();
        let screen_h = screen_height();
        let grid_w = screen_w - SIDEBAR_WIDTH;
        let cell_w = grid_w / GRID_WIDTH as f32;
        let cell_h = screen_h / GRID_HEIGHT as f32;

        // ── Input ──
        handle_grid_input(&mut state, grid_w, cell_w, cell_h);

        // ── UI ──
        ui.draw(&mut state, grid_w, screen_w, screen_h);

        // ── Grid rendering ──
        draw_grid(&state.grid, cell_w, cell_h);

        next_frame().await;
    }
}

fn handle_grid_input(state: &mut AppState, grid_w: f32, cell_w: f32, cell_h: f32) {
    if is_mouse_button_down(MouseButton::Left) {
        let (mx, my) = mouse_position();
        if mx < grid_w {
            let col = (mx / cell_w) as usize;
            let row = (my / cell_h) as usize;
            if col < GRID_WIDTH && row < GRID_HEIGHT {
                let idx = state.grid.idx(row, col);
                match state.active_tool {
                    Tool::Wall  => state.grid.set_wall(idx, true),
                    Tool::Erase => state.grid.set_wall(idx, false),
                    Tool::Mud   => state.grid.set_terrain(idx, COST_MUD),
                    Tool::Water => state.grid.set_terrain(idx, COST_WATER),
                }
            }
        }
    }

    if is_mouse_button_down(MouseButton::Right) {
        let (mx, my) = mouse_position();
        if mx < grid_w {
            let col = (mx / cell_w) as usize;
            let row = (my / cell_h) as usize;
            if col < GRID_WIDTH && row < GRID_HEIGHT {
                let idx = state.grid.idx(row, col);
                state.grid.set_wall(idx, false);
            }
        }
    }

    if is_key_pressed(KeyCode::R) { state.reset_all(); }
}

fn draw_grid(grid: &Grid, cell_w: f32, cell_h: f32) {
    for i in 0..(grid.width * grid.height) {
        let col = i % grid.width;
        let row = i / grid.width;
        let cell = grid.cell(i);

        let color = if cell.is_wall         { color_u8!(69, 71, 90, 255) }
            else if cell.is_start           { color_u8!(166, 227, 161, 255) }
            else if cell.is_end             { color_u8!(243, 139, 168, 255) }
            else if cell.is_path            { color_u8!(249, 226, 175, 255) }
            else if cell.is_visited         { color_u8!(137, 180, 250, 255) }
            else if cell.cost == grid::COST_MUD   { color_u8!(161, 138, 90, 255) }
            else if cell.cost == grid::COST_WATER { color_u8!(90, 138, 161, 255) }
            else                            { color_u8!(30, 30, 46, 255) };

        draw_rectangle(
            col as f32 * cell_w,
            row as f32 * cell_h,
            cell_w,
            cell_h,
            color,
        );
    }

    // Grid lines
    if cell_w > 6.0 {
        for r in 0..=grid.height {
            draw_line(0.0, r as f32 * cell_h, grid.width as f32 * cell_w, r as f32 * cell_h, 0.5, color_u8!(0, 0, 0, 20));
        }
        for c in 0..=grid.width {
            draw_line(c as f32 * cell_w, 0.0, c as f32 * cell_w, grid.height as f32 * cell_h, 0.5, color_u8!(0, 0, 0, 20));
        }
    }
}