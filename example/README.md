# BLS Encryption Web Example

This is a Next.js web application that demonstrates the BLS encryption library compiled to WebAssembly. It provides a simple interface for encrypting and decrypting files using BLS cryptography.

## Features

- **Key Generation**: Generate new BLS keypairs with a single click
- **File Encryption**: Upload any file and encrypt it using a public key
- **File Decryption**: Decrypt encrypted files using the corresponding secret key
- **Hash Validation**: Verify data integrity using SHA256 checksums
- **WebAssembly**: Runs the Rust BLS encryption library directly in the browser

## Setup

1. **Install dependencies**:
   ```bash
   npm install
   ```

2. **Build the WASM module** (from the project root):
   ```bash
   ./build-wasm.sh
   ```

3. **Run the development server**:
   ```bash
   npm run dev
   ```

4. Open [http://localhost:3000](http://localhost:3000) in your browser.

## Usage

### 1. Generate Keys
- Click "Generate New Keypair" to create a new BLS keypair
- The public key is used for encryption
- The secret key is used for decryption (keep it private!)
- Each keypair has a unique Key ID for identification

### 2. Encrypt Files
- Select any file using the file picker
- Click "Encrypt File" to encrypt it with the generated public key
- The encrypted file will be automatically downloaded as a JSON file
- The original file's SHA256 hash is included for integrity verification

### 3. Decrypt Files
- Upload the encrypted JSON file
- Paste the secret key that corresponds to the public key used for encryption
- Optionally provide the expected hash for validation
- Click "Decrypt File" to decrypt and download the original file
- Hash validation will show if the decrypted data matches the expected checksum

## File Format

Encrypted files are saved as JSON with the following structure:
```json
{
  "encrypted_data": "base64-encoded-encrypted-content",
  "data_hash": "sha256-hash-of-original-data",
  "public_key_id": "identifier-of-public-key-used"
}
```

## Security Notes

- **Keep secret keys private**: Never share or expose secret keys
- **Verify hashes**: Always check the hash validation when decrypting important data
- **Use HTTPS**: In production, ensure all communication is over HTTPS
- **Key management**: Implement proper key storage and management for production use

## Technical Details

- Built with Next.js 16 and React 19
- Uses Tailwind CSS for styling
- WebAssembly module compiled from Rust using wasm-pack
- Implements hybrid BLS + AES encryption for performance
- SHA256 hashing for data integrity verification

## Development

The WASM module is currently mocked for development purposes. To use the actual Rust implementation:

1. Ensure you have `wasm-pack` installed
2. Run the build script to compile the Rust code to WebAssembly
3. Update the `useBLSWasm` hook to load the actual WASM module instead of the mock

## Limitations

- This is a demonstration/example implementation
- Not suitable for production use without additional security measures
- File size is limited by browser memory constraints
- No key persistence or management features