// WASM loader for BLS encryption service
let wasmModule: any = null;

export async function loadBLSWasm() {
  if (wasmModule) {
    return wasmModule;
  }

  try {
    // Load WASM module using dynamic import with full URL
    const wasmUrl = new URL('/wasm/bls-encryption.js', window.location.origin);
    
    // Use dynamic import with the full URL
    const module = await import(/* webpackIgnore: true */ wasmUrl.href);
    
    // Initialize the WASM module
    await module.default();
    
    wasmModule = {
      BLSWasm: module.BLSWasm,
      default: module.default
    };
    
    return wasmModule;
  } catch (error) {
    console.error('Failed to load WASM module:', error);
    throw error;
  }
}

// Alternative loader using fetch + eval for problematic environments
export async function loadBLSWasmFallback() {
  if (wasmModule) {
    return wasmModule;
  }

  try {
    // Fetch the WASM JS file
    const response = await fetch('/wasm/bls-encryption.js');
    if (!response.ok) {
      throw new Error(`Failed to fetch WASM JS: ${response.statusText}`);
    }
    
    const jsCode = await response.text();
    
    // Create a data URL for the module
    const blob = new Blob([jsCode], { type: 'application/javascript' });
    const moduleUrl = URL.createObjectURL(blob);
    
    try {
      // Import the module from the blob URL
      const module = await import(/* webpackIgnore: true */ moduleUrl);
      
      // Initialize the WASM module
      await module.default();
      
      wasmModule = {
        BLSWasm: module.BLSWasm,
        default: module.default
      };
      
      return wasmModule;
    } finally {
      URL.revokeObjectURL(moduleUrl);
    }
  } catch (error) {
    console.error('Failed to load WASM module with fallback:', error);
    throw error;
  }
}
