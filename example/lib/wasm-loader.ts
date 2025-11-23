// WASM loader for BLS encryption service
let wasmModule: any = null;

export async function loadBLSWasm() {
  if (wasmModule) {
    return wasmModule;
  }

  // Try multiple paths for WASM files
  const wasmPaths = [
    '/wasm/bls-encryption.js',  // Original path in public/wasm/
    './bls-encryption.js'       // Direct path in lib/ directory
  ];

  for (const path of wasmPaths) {
    try {
      let module;
      
      if (path.startsWith('./')) {
        // Direct import for local files
        module = await import(/* webpackIgnore: true */ path);
      } else {
        // Load WASM module using dynamic import with full URL
        const wasmUrl = new URL(path, window.location.origin);
        module = await import(/* webpackIgnore: true */ wasmUrl.href);
      }
      
      // Initialize the WASM module
      await module.default();
      
      wasmModule = {
        BLSWasm: module.BLSWasm,
        default: module.default
      };
      
      return wasmModule;
    } catch (error) {
      console.warn(`Failed to load WASM from ${path}:`, error);
      // Continue to next path
    }
  }
  
  throw new Error('Failed to load WASM module from any path');
}

// Alternative loader using fetch + eval for problematic environments
export async function loadBLSWasmFallback() {
  if (wasmModule) {
    return wasmModule;
  }

  // Try multiple paths for fallback loading
  const wasmPaths = ['/wasm/bls-encryption.js', '/lib/bls-encryption.js'];

  for (const path of wasmPaths) {
    try {
      // Fetch the WASM JS file
      const response = await fetch(path);
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
      console.warn(`Failed to load WASM fallback from ${path}:`, error);
      // Continue to next path
    }
  }
  
  throw new Error('Failed to load WASM module with fallback from any path');
}
