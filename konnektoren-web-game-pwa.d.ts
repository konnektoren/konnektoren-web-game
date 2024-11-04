/* tslint:disable */
/* eslint-disable */
/**
 * @param {string} root_selector
 */
export function render_app(root_selector: string): void;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly render_app: (a: number, b: number) => void;
  readonly main: (a: number, b: number) => number;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __wbindgen_export_2: WebAssembly.Table;
  readonly __wbindgen_export_3: WebAssembly.Table;
  readonly _dyn_core__ops__function__FnMut_____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__he16461dfd15748d3: (a: number, b: number) => void;
  readonly closure2422_externref_shim: (a: number, b: number, c: number) => void;
  readonly _dyn_core__ops__function__Fn__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h8a3309b8be22bb05: (a: number, b: number, c: number, d: number) => void;
  readonly closure4373_externref_shim: (a: number, b: number, c: number) => void;
  readonly _dyn_core__ops__function__Fn__A_B___Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__hf4f1dd2cbe341d2f: (a: number, b: number, c: number, d: number, e: number, f: number) => void;
  readonly _dyn_core__ops__function__FnMut_____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__hba4a7cf74fd62ec2: (a: number, b: number) => void;
  readonly _dyn_core__ops__function__FnMut_____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__hc7809db837c0c9c6: (a: number, b: number) => void;
  readonly closure5232_externref_shim: (a: number, b: number, c: number) => void;
  readonly closure5296_externref_shim: (a: number, b: number, c: number) => void;
  readonly closure6097_externref_shim: (a: number, b: number, c: number) => number;
  readonly _dyn_core__ops__function__FnMut_____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__hfa2feec3b5c84458: (a: number, b: number) => void;
  readonly closure6703_externref_shim: (a: number, b: number, c: number) => void;
  readonly closure6744_externref_shim: (a: number, b: number, c: number) => void;
  readonly __wbindgen_exn_store: (a: number) => void;
  readonly __externref_table_alloc: () => number;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
  readonly __externref_drop_slice: (a: number, b: number) => void;
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
