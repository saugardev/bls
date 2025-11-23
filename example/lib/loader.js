import init, { BLSWasm } from './bls-encryption.js';

let wasmModule = null;

export async function loadBLSWasm() {
  if (!wasmModule) {
    await init();
    wasmModule = BLSWasm;
  }
  return wasmModule;
}
