/* tslint:disable */
/* eslint-disable */
/**
* @param {string} input
* @param {ConvertOptions} options
* @returns {string}
*/
export function candidates(input: string, options: ConvertOptions): string;
/**
*/
export class ConvertOptions {
  free(): void;
/**
*/
  constructor();
/**
* @param {boolean} value
*/
  dont_match_newlines: boolean;
/**
* @param {boolean} value
*/
  print_equals: boolean;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_convertoptions_free: (a: number) => void;
  readonly convertoptions_new: () => number;
  readonly convertoptions_set_dont_match_newlines: (a: number, b: number) => void;
  readonly convertoptions_set_print_equals: (a: number, b: number) => void;
  readonly candidates: (a: number, b: number, c: number, d: number) => void;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  readonly __wbindgen_malloc: (a: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number) => number;
  readonly __wbindgen_free: (a: number, b: number) => void;
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
