/* tslint:disable */
/* eslint-disable */
/**
*/
export class Othello {
  free(): void;
/**
* @returns {boolean} 
*/
  finish(): boolean;
/**
* @returns {Othello} 
*/
  static new(): Othello;
/**
* @param {boolean} white_is_com 
* @param {boolean} black_is_com 
* @param {number} depth 
*/
  start(white_is_com: boolean, black_is_com: boolean, depth: number): void;
/**
* @returns {number} 
*/
  get_winner(): number;
/**
*/
  draw(): void;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_othello_free: (a: number) => void;
  readonly othello_finish: (a: number) => number;
  readonly othello_new: () => number;
  readonly othello_start: (a: number, b: number, c: number, d: number) => void;
  readonly othello_get_winner: (a: number) => number;
  readonly othello_draw: (a: number) => void;
  readonly __wbindgen_malloc: (a: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number) => number;
  readonly __wbindgen_exn_store: (a: number) => void;
}

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
        