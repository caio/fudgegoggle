/* tslint:disable */
/* eslint-disable */
/**
* @param {number} width
* @param {number} height
* @param {Uint8Array} src
* @returns {string}
*/
export function qr_decode(width: number, height: number, src: Uint8Array): string;
/**
* @param {string} input
* @returns {string}
*/
export function decode_otpauth(input: string): string;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly qr_decode: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly decode_otpauth: (a: number, b: number, c: number) => void;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  readonly __wbindgen_malloc: (a: number) => number;
  readonly __wbindgen_free: (a: number, b: number) => void;
  readonly __wbindgen_realloc: (a: number, b: number, c: number) => number;
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
