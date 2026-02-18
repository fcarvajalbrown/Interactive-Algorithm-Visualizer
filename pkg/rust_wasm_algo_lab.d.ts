/* tslint:disable */
/* eslint-disable */

/**
 * Top-level WASM entry point.
 * AlgoLab owns the Grid and Stats, and orchestrates algorithm execution.
 * JS only talks to this struct — it never directly instantiates Grid or Stats.
 */
export class AlgoLab {
    free(): void;
    [Symbol.dispose](): void;
    generate_maze_backtracker(): void;
    generate_maze_prims(): void;
    constructor(width: number, height: number);
    render_buffer_len(): number;
    render_buffer_ptr(): number;
    reset_all(): void;
    reset_search(): void;
    run_astar(): AlgoResult;
    run_bfs(): AlgoResult;
    run_dfs(): AlgoResult;
    run_dijkstra(): AlgoResult;
    set_end(idx: number): void;
    set_start(idx: number): void;
    set_terrain(idx: number, cost: number): void;
    set_wall(idx: number, active: boolean): void;
}

/**
 * Returned to JS after each algorithm run.
 * wasm_bindgen can't return structs with complex fields directly,
 * so we use a flat getter-based struct instead.
 */
export class AlgoResult {
    private constructor();
    free(): void;
    [Symbol.dispose](): void;
    execution_ms(): number;
    nodes_explored(): number;
    path_found(): boolean;
    path_length(): number;
}

export class Grid {
    free(): void;
    [Symbol.dispose](): void;
    flush_render_buffer(): void;
    constructor(width: number, height: number);
    render_buffer_len(): number;
    render_buffer_ptr(): number;
    reset_all(): void;
    reset_search_state(): void;
    set_end(idx: number): void;
    set_start(idx: number): void;
    set_terrain(idx: number, cost: number): void;
    set_wall(idx: number, value: boolean): void;
    height: number;
    width: number;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
    readonly memory: WebAssembly.Memory;
    readonly __wbg_algolab_free: (a: number, b: number) => void;
    readonly __wbg_algoresult_free: (a: number, b: number) => void;
    readonly algolab_generate_maze_backtracker: (a: number) => void;
    readonly algolab_generate_maze_prims: (a: number) => void;
    readonly algolab_new: (a: number, b: number) => number;
    readonly algolab_render_buffer_len: (a: number) => number;
    readonly algolab_render_buffer_ptr: (a: number) => number;
    readonly algolab_reset_all: (a: number) => void;
    readonly algolab_reset_search: (a: number) => void;
    readonly algolab_run_astar: (a: number) => number;
    readonly algolab_run_bfs: (a: number) => number;
    readonly algolab_run_dfs: (a: number) => number;
    readonly algolab_run_dijkstra: (a: number) => number;
    readonly algolab_set_end: (a: number, b: number) => void;
    readonly algolab_set_start: (a: number, b: number) => void;
    readonly algolab_set_terrain: (a: number, b: number, c: number) => void;
    readonly algolab_set_wall: (a: number, b: number, c: number) => void;
    readonly algoresult_execution_ms: (a: number) => number;
    readonly algoresult_nodes_explored: (a: number) => number;
    readonly algoresult_path_found: (a: number) => number;
    readonly algoresult_path_length: (a: number) => number;
    readonly __wbg_get_grid_height: (a: number) => number;
    readonly __wbg_get_grid_width: (a: number) => number;
    readonly __wbg_grid_free: (a: number, b: number) => void;
    readonly __wbg_set_grid_height: (a: number, b: number) => void;
    readonly __wbg_set_grid_width: (a: number, b: number) => void;
    readonly grid_flush_render_buffer: (a: number) => void;
    readonly grid_new: (a: number, b: number) => number;
    readonly grid_render_buffer_len: (a: number) => number;
    readonly grid_render_buffer_ptr: (a: number) => number;
    readonly grid_reset_all: (a: number) => void;
    readonly grid_reset_search_state: (a: number) => void;
    readonly grid_set_end: (a: number, b: number) => void;
    readonly grid_set_start: (a: number, b: number) => void;
    readonly grid_set_terrain: (a: number, b: number, c: number) => void;
    readonly grid_set_wall: (a: number, b: number, c: number) => void;
    readonly __wbindgen_exn_store: (a: number) => void;
    readonly __externref_table_alloc: () => number;
    readonly __wbindgen_externrefs: WebAssembly.Table;
    readonly __wbindgen_free: (a: number, b: number, c: number) => void;
    readonly __wbindgen_malloc: (a: number, b: number) => number;
    readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
    readonly __wbindgen_start: () => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;

/**
 * Instantiates the given `module`, which can either be bytes or
 * a precompiled `WebAssembly.Module`.
 *
 * @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
 *
 * @returns {InitOutput}
 */
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
 * If `module_or_path` is {RequestInfo} or {URL}, makes a request and
 * for everything else, calls `WebAssembly.instantiate` directly.
 *
 * @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
 *
 * @returns {Promise<InitOutput>}
 */
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
