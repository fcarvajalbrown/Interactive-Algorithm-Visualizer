// src/ui.rs
use macroquad::prelude::*;
use crate::{AppState, Tool, SIDEBAR_WIDTH, GRID_WIDTH, GRID_HEIGHT};

const BTN_H: f32 = 30.0;
const BTN_MARGIN: f32 = 6.0;
const SECTION_MARGIN: f32 = 16.0;
const TEXT_SIZE: f32 = 14.0;

// Catppuccin Mocha
const BG:       Color = color_u8!(24, 24, 37, 255);
const SURFACE:  Color = color_u8!(30, 30, 46, 255);
const OVERLAY:  Color = color_u8!(49, 50, 68, 255);
const TEXT:     Color = color_u8!(205, 214, 244, 255);
const SUBTEXT:  Color = color_u8!(166, 173, 200, 255);
const ACCENT:   Color = color_u8!(137, 180, 250, 255);
const RED:      Color = color_u8!(243, 139, 168, 255);
const GREEN:    Color = color_u8!(166, 227, 161, 255);
const YELLOW:   Color = color_u8!(249, 226, 175, 255);

pub struct Ui {
    pub hovered: Option<usize>,
}

impl Ui {
    pub fn new() -> Self {
        Self { hovered: None }
    }

    pub fn draw(&mut self, state: &mut AppState, grid_w: f32, _screen_w: f32, screen_h: f32) {
        // Sidebar background
        draw_rectangle(grid_w, 0.0, SIDEBAR_WIDTH, screen_h, BG);
        draw_line(grid_w, 0.0, grid_w, screen_h, 1.0, OVERLAY);

        let x = grid_w + 12.0;
        let w = SIDEBAR_WIDTH - 24.0;
        let mut y = 16.0;

        // Title
        draw_text("Algorithm Lab", x, y + TEXT_SIZE, TEXT_SIZE + 2.0, ACCENT);
        y += TEXT_SIZE + SECTION_MARGIN + 4.0;

        // Algorithms
        y = self.section_label("ALGORITHMS", x, y);
        for name in ["BFS", "DFS", "Dijkstra", "A*"] {
            let active = state.last_algo == Some(name);
            if self.button(name, x, y, w, active, ACCENT) {
                state.run_algo(name);
            }
            y += BTN_H + BTN_MARGIN;
        }
        y += SECTION_MARGIN;

        // Maze
        y = self.section_label("MAZE", x, y);
        if self.button("Recursive Backtracker", x, y, w, false, ACCENT) {
        state.grid.reset_all();
        maze::recursive_backtracker::generate(&mut state.grid);
        }
        y += BTN_H + BTN_MARGIN;
        if self.button("Prim's", x, y, w, false, ACCENT) {
        maze::prims::generate(&mut state.grid);
        state.grid.set_start(state.grid.idx(1, 1));
        state.grid.set_end(state.grid.idx(GRID_HEIGHT - 2, GRID_WIDTH - 2));
        }
        y += BTN_H + BTN_MARGIN + SECTION_MARGIN;

        // Tools
        y = self.section_label("TOOLS", x, y);
        for (label, tool, color) in [
            ("Wall",  Tool::Wall,  OVERLAY),
            ("Erase", Tool::Erase, OVERLAY),
            ("Mud",   Tool::Mud,   color_u8!(161, 138, 90, 255)),
            ("Water", Tool::Water, color_u8!(90, 138, 161, 255)),
        ] {
            let active = state.active_tool == tool;
            if self.button(label, x, y, w, active, color) {
                state.active_tool = tool;
            }
            y += BTN_H + BTN_MARGIN;
        }
        y += SECTION_MARGIN;

        // Controls
        y = self.section_label("CONTROLS", x, y);
        if self.button("Reset Search", x, y, w, false, ACCENT) {
            state.reset_search();
        }
        y += BTN_H + BTN_MARGIN;
        if self.button("Reset All (R)", x, y, w, false, RED) {
            state.reset_all();
        }
        y += BTN_H + BTN_MARGIN + SECTION_MARGIN;

        // Stats
        y = self.section_label("STATS", x, y);
        draw_text(&format!("Nodes: {}", state.stats.nodes_explored), x, y + TEXT_SIZE, TEXT_SIZE, SUBTEXT);
        y += TEXT_SIZE + BTN_MARGIN;
        let path_str = if state.stats.path_found {
            format!("Path: {}", state.stats.path_length)
        } else {
            "Path: -".to_string()
        };
        draw_text(&path_str, x, y + TEXT_SIZE, TEXT_SIZE, SUBTEXT);
        y += TEXT_SIZE + BTN_MARGIN;
        draw_text(&format!("Time: {:.2}ms", state.stats.execution_ms), x, y + TEXT_SIZE, TEXT_SIZE, YELLOW);
    }

    fn section_label(&self, label: &str, x: f32, y: f32) -> f32 {
        draw_text(label, x, y + TEXT_SIZE, TEXT_SIZE - 2.0, SUBTEXT);
        y + TEXT_SIZE + BTN_MARGIN
    }

    fn button(&mut self, label: &str, x: f32, y: f32, w: f32, active: bool, color: Color) -> bool {
        let (mx, my) = mouse_position();
        let hovered = mx >= x && mx <= x + w && my >= y && my <= y + BTN_H;
        let clicked = hovered && is_mouse_button_pressed(MouseButton::Left);

        let bg = if active       { color }
                 else if hovered { OVERLAY }
                 else            { SURFACE };

        draw_rectangle(x, y, w, BTN_H, bg);
        draw_rectangle_lines(x, y, w, BTN_H, 1.0, if hovered || active { color } else { OVERLAY });
        draw_text(label, x + 8.0, y + BTN_H / 2.0 + TEXT_SIZE / 2.0 - 2.0, TEXT_SIZE, if active { SURFACE } else { TEXT });

        clicked
    }
}

use crate::maze;