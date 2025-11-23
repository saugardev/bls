// WASM loader for BLS encryption
let wasmModule = null;
let BLSWasm = null;

export async function loadBLSWasm() {
  if (wasmModule && BLSWasm) {
    return { BLSWasm };
  }

  try {
    // Dynamically load the WASM module using a script tag approach
    const script = document.createElement('script');
    script.type = 'module';
    script.textContent = `
      import init, { BLSWasm as WasmBLSWasm } from '/bls-encryption.js';
      
      window.__wasmInit = init;
      window.__BLSWasm = WasmBLSWasm;
    `;
    
    document.head.appendChild(script);
    
    // Wait for the script to load
    await new Promise((resolve, reject) => {
      const checkLoaded = () => {
        if (window.__wasmInit && window.__BLSWasm) {
          resolve();
        } else {
          setTimeout(checkLoaded, 100);
        }
      };
      checkLoaded();
      
      // Timeout after 10 seconds
      setTimeout(() => reject(new Error('WASM loading timeout')), 10000);
    });
    
    // Initialize the WASM module
    await window.__wasmInit();
    
    BLSWasm = window.__BLSWasm;
    wasmModule = { BLSWasm };
    
    // Clean up
    document.head.removeChild(script);
    delete window.__wasmInit;
    delete window.__BLSWasm;
    
    return wasmModule;
  } catch (error) {
    console.error('Failed to load WASM module:', error);
    throw error;
  }
}