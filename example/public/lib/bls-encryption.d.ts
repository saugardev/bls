/* tslint:disable */
/* eslint-disable */
export function main(): void;
export class BLSWasm {
  free(): void;
  [Symbol.dispose](): void;
  constructor();
  /**
   * Generate a new keypair
   */
  generate_keypair(): string;
  /**
   * Encrypt data with a public key
   */
  encrypt(data: Uint8Array, public_key_b64: string): string;
  /**
   * Decrypt data with a secret key
   */
  decrypt(encrypted_data_b64: string, secret_key_b64: string): string;
  /**
   * Validate hash against decrypted data
   */
  validate_hash(data: Uint8Array, expected_hash: string): boolean;
  /**
   * Compute SHA256 hash of data
   */
  compute_hash(data: Uint8Array): string;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_blswasm_free: (a: number, b: number) => void;
  readonly blswasm_new: () => number;
  readonly blswasm_generate_keypair: (a: number) => [number, number, number, number];
  readonly blswasm_encrypt: (a: number, b: number, c: number, d: number, e: number) => [number, number, number, number];
  readonly blswasm_decrypt: (a: number, b: number, c: number, d: number, e: number) => [number, number, number, number];
  readonly blswasm_validate_hash: (a: number, b: number, c: number, d: number, e: number) => number;
  readonly blswasm_compute_hash: (a: number, b: number, c: number) => [number, number];
  readonly main: () => void;
  readonly __wbindgen_exn_store: (a: number) => void;
  readonly __externref_table_alloc: () => number;
  readonly __wbindgen_externrefs: WebAssembly.Table;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __externref_table_dealloc: (a: number) => void;
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
