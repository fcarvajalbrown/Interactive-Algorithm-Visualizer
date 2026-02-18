# Rust WASM Algorithm Lab — Decisions Log

## Architecture Decisions

### No Leptos / No Dioxus
Leptos and Dioxus are reactive UI frameworks designed for component trees and DOM diffing. This project renders a 200x200 grid (~40,000 cells) at 60 FPS on a Canvas element, which bypasses the DOM entirely. Reactive signals have no value here — there are no components to diff. The frameworks add compile time, bundle size, and an awkward ownership split between the framework's render cycle and the Canvas's requestAnimationFrame loop. Vanilla JS calling into WASM directly is simpler, faster, and better demonstrates Rust's systems value without framework noise.
**Chosen:** Vanilla JS with no framework.

---

### Render Ownership
Rust owns all grid state and algorithm logic. JS owns the canvas render loop. JS reads directly from WASM memory buffer via Uint8Array — zero copy. This keeps the hot path out of Rust-JS call overhead.
**Chosen:** Rust owns state, JS owns rendering.

---

### Cell Representation
Struct with flags chosen over u8 enum or bitfield for flexibility. Weighted terrain (mud, water) and multiple boolean states (visited, path, wall, start, end) are cleanly expressed as named fields.
**Chosen:** Cell struct with bool flags and u16 cost.

---

### Render Buffer Contract
Grid exposes a flat u8 render buffer readable by JS via shared WASM memory pointer. `flush_render_buffer()` is called explicitly after mutations so JS sees incremental updates each frame. `to_render_byte()` is the single encoding contract between Rust and JS — adding a new cell type only requires updating this method and the CELL_COLORS map in JS. Single point of change.

Encoding:
- 0 = empty
- 1 = wall
- 2 = start
- 3 = end
- 4 = visited
- 5 = path
- 6 = mud
- 7 = water

**Chosen:** Shared memory buffer, explicit flush, u8 encoding matched by CELL_COLORS in JS.

---

### wasm_bindgen Split impl Blocks
`wasm_bindgen` cannot cross the WASM boundary with references to custom structs like `&Cell` or `&mut Cell`. Grid was split into two impl blocks: one with `#[wasm_bindgen]` for JS-facing methods, one plain impl for internal Rust-only methods (`cell`, `cell_mut`, `neighbors`, `idx`). This avoids `WasmDescribe` trait errors.
**Chosen:** Split impl blocks on Grid.

---

### AlgoResult Flat Getters
`wasm_bindgen` cannot serialize structs with complex fields directly. `AlgoResult` uses flat getter methods instead of returning a JS object to avoid runtime surprises.
**Chosen:** Flat getter methods on AlgoResult.

---

### Shared reconstruct_path
`reconstruct_path` was identical in all 4 algorithm files. Duplication means any bug fix or change must be applied 4 times. Moved to `algorithms/mod.rs` as a shared utility. All algorithms import via `super::reconstruct_path`. If you ever need to change path reconstruction logic, you change it in one place.
**Chosen:** Shared in algorithms/mod.rs, imported via `super::`.

---

### Maze 2-Step Neighbor Traversal
Maze generation uses neighbors 2 steps away, not 1. This preserves wall structure — you always carve through the cell between current and neighbor. Using 1-step neighbors would produce an open field instead of a maze.
**Chosen:** 2-step neighbor traversal in maze generators.

---

### Algorithm Explanations

**BFS (Breadth-First Search)**
Uses a queue (FIFO). Explores all neighbors at the current depth before going deeper. Guarantees the shortest path in an unweighted grid. Visits many nodes but finds the optimal route.

**DFS (Depth-First Search)**
Uses a stack (LIFO). Explores as deep as possible before backtracking. Does NOT guarantee the shortest path. Faster to find *a* path but it may be suboptimal. The only difference from BFS is `VecDeque::pop_front()` → `Vec::pop()`.

**Dijkstra**
Uses a min-heap ordered by cumulative cost. Respects weighted terrain (mud/water). Guarantees the shortest path in a weighted grid. Radiates outward from start equally in all directions weighted by cost.

**A\***
Uses a min-heap ordered by `f = g + h` where `g` is the actual cost from start and `h` is the Manhattan distance heuristic to the end. The heuristic biases exploration toward the goal, so it explores dramatically fewer nodes than Dijkstra on open grids. Manhattan is the correct heuristic for 4-directional movement — it never overestimates, keeping A* optimal.

**Recursive Backtracker (maze)**
Starts with a fully walled grid. Uses a stack (DFS) to carve passages. Picks a random unvisited neighbor, knocks down the wall between them, and continues. Produces mazes that feel like one long winding path — few branches, long dead ends.

**Prim's (maze)**
Starts with a fully walled grid. Maintains a frontier of all reachable walls and picks one randomly. Produces mazes with more branching and shorter dead ends. Feels like a blob expanding outward rather than a snake carving through.

---

### No ES Modules / Single app.js
Trunk on Windows has a confirmed bug where it intercepts all requests and returns index.html with MIME type text/html, blocking ES module loading. Rewriting to a single app.js with no import/export statements bypasses this entirely. Eventually migrated away from Trunk entirely to wasm-pack + hand-written index.html served statically.
**Chosen:** Single app.js, no ES modules, wasm-pack for builds.

---

## Deferred Fixes

### Duplicate CELL_COLORS
`CELL_COLORS` is duplicated in `app.js` and `comparison.js`. Extract to a shared constant once comparison mode is wired up.

### Duplicate Maze Utils
`carve()`, `maze_neighbors()`, and `random_index()` are duplicated in `recursive_backtracker.rs` and `prims.rs`. Extract to `maze/mod.rs` after core functionality is confirmed working.

### Hashed WASM Filename
Trunk generates a hashed filename for the WASM JS bindings on some builds. If hash reappears, check `dist/` and update the import accordingly. Fixed by switching to wasm-pack which outputs stable filenames.

---

## Mistakes Made

### Partial File Edits
When given full rewrites, partial edits were applied instead of full file replacements, leaving old method names mixed with new ones. Always full-replace when given a complete rewrite.

### Missing maze/mod.rs
`src/maze/mod.rs` was not created. Rust requires an explicit `mod.rs` for every folder acting as a module.

### Maze Files Not Updated
Maze files were incorrectly marked as unchanged when they still referenced the old `get_mut` API that had just been renamed to `cell_mut`. Cost several build cycles.

### Claude Marked Files as Unchanged Incorrectly
When rewriting the algorithm files to use `cell`/`cell_mut`, Claude incorrectly stated the maze files were "unchanged, already correct" when they were not. This caused multiple wasted build cycles.