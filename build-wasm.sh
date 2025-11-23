#!/bin/bash

# Build the WASM module using wasm-pack
echo "Building BLS Encryption WASM module..."

# Install wasm-pack if not available
if ! command -v wasm-pack &> /dev/null; then
    echo "Installing wasm-pack..."
    curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
fi

# Build the WASM package with only WASM-compatible features
wasm-pack build --target web --out-dir example/public/wasm --out-name bls-encryption --no-default-features --features wasm

# Copy the generated files to the Next.js public directory
echo "WASM module built successfully!"
echo "Files generated in example/public/wasm/"

# Create a simple loader for the WASM module
cat > example/public/wasm/loader.js << 'EOF'
import init, { BLSWasm } from './bls-encryption.js';

let wasmModule = null;

export async function loadBLSWasm() {
  if (!wasmModule) {
    await init();
    wasmModule = BLSWasm;
  }
  return wasmModule;
}
EOF

echo "WASM loader created at example/public/wasm/loader.js"
