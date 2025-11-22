# BLS Encryption Service

A high-performance Rust-based BLS encryption/decryption service that provides hybrid encryption (BLS for key encapsulation, AES-256-GCM for data encryption).

## Features

- **Hybrid Encryption**: BLS + AES-256-GCM for optimal security and performance
- **High Performance**: Encrypts 1GB in <2 seconds
- **CLI Tool**: Easy-to-use command-line interface
- **REST API**: HTTP service for remote encryption/decryption
- **File Support**: Handles files of any size efficiently
- **Data Integrity**: SHA256 verification of encrypted data
- **Key Management**: Secure keypair generation and storage

## Quick Start

### Build the Project

```bash
cargo build --release
```

### Generate Keys

```bash
./target/release/bls-encryption-service generate-keys --output ./keys --name alice
```

### Encrypt a File

```bash
./target/release/bls-encryption-service encrypt \
    --input document.pdf \
    --public-key ./keys/alice_public.key \
    --output encrypted.bin
```

### Decrypt a File

```bash
./target/release/bls-encryption-service decrypt \
    --input encrypted.bin \
    --secret-key ./keys/alice_secret.key \
    --output decrypted.pdf
```

### Encrypt Text

```bash
./target/release/bls-encryption-service encrypt \
    --text "Secret message" \
    --public-key ./keys/alice_public.key \
    --output message.enc
```

### Run Benchmarks

```bash
./target/release/bls-encryption-service benchmark --size-mb 100
```

### Start HTTP Service

```bash
./target/release/bls-encryption-service service --port 3000 --secret-key ./keys/alice_secret.key
```

## API Endpoints

### POST /encrypt
Encrypt data using a public key.

**Request:**
```json
{
  "data": "SGVsbG8gV29ybGQ=",  // base64 encoded data
  "public_key": "..."          // base64 encoded public key
}
```

**Response:**
```json
{
  "encrypted_data": {
    "encrypted_aes_key": "...",
    "encrypted_content": "...",
    "nonce": "...",
    "data_hash": "...",
    "public_key_id": "..."
  },
  "success": true,
  "message": "Data encrypted successfully"
}
```

### POST /decrypt
Decrypt data using a secret key.

**Request:**
```json
{
  "encrypted_data": { /* EncryptedData object */ },
  "secret_key": "..."  // base64 encoded secret key
}
```

**Response:**
```json
{
  "data": "SGVsbG8gV29ybGQ=",  // base64 encoded decrypted data
  "success": true,
  "message": "Data decrypted successfully"
}
```

### GET /pubkey
Get the service's public key (if configured with a secret key).

**Response:**
```json
{
  "public_key": "...",  // base64 encoded
  "key_id": "..."
}
```

### GET /health
Health check endpoint.

**Response:**
```json
{
  "status": "healthy",
  "service": "bls-encryption-service",
  "version": "0.1.0"
}
```

## Performance

Benchmarked on modern hardware:

- **Keypair Generation**: <100ms
- **1MB Encryption**: <50ms
- **100MB Encryption**: <500ms
- **1GB Encryption**: <2s
- **Decryption**: Similar speed to encryption

## Security

- **BLS Signatures**: Uses the BLST library for BLS operations
- **AES-256-GCM**: Industry-standard symmetric encryption
- **Key Encapsulation**: Secure hybrid encryption scheme
- **Data Integrity**: SHA256 hash verification
- **Secure Random**: Cryptographically secure random number generation

## Architecture

```
┌─────────────────┐    ┌──────────────────┐    ┌─────────────────┐
│   Input Data    │───▶│  AES-256-GCM     │───▶│ Encrypted Data  │
└─────────────────┘    │  (Random Key)    │    └─────────────────┘
                       └──────────────────┘
                              │
                              ▼
                       ┌──────────────────┐    ┌─────────────────┐
                       │   BLS Encrypt    │───▶│ Encrypted Key   │
                       │   (Public Key)   │    └─────────────────┘
                       └──────────────────┘
```

## Testing

Run the test suite:

```bash
cargo test
```

Run integration tests:

```bash
cargo test --test encryption_test
```

## Examples

### Basic Usage

```rust
use bls_encryption_service::{BLSEncryption, KeyManager};

// Generate keypair
let keypair = KeyManager::generate_keypair()?;

// Create encryption instance
let encryption = BLSEncryption::new();

// Encrypt data
let data = b"Hello, BLS!";
let encrypted = encryption.encrypt(data, &keypair.public_key)?;

// Decrypt data
let decrypted = encryption.decrypt(&encrypted, &keypair.secret_key)?;
assert_eq!(data, decrypted.as_slice());
```

### File Encryption

```rust
use std::fs::File;
use std::io::{BufReader, BufWriter};

let input_file = File::open("document.pdf")?;
let output_file = File::create("encrypted.bin")?;

let reader = BufReader::new(input_file);
let writer = BufWriter::new(output_file);

let encrypted_data = encryption.encrypt_large(reader, writer, &public_key)?;
```

## Dependencies

- **blst**: BLS signature library
- **aes-gcm**: AES-GCM encryption
- **sha2**: SHA256 hashing
- **clap**: CLI argument parsing
- **axum**: HTTP service framework
- **tokio**: Async runtime
- **serde**: Serialization
- **base64**: Base64 encoding
- **rand**: Random number generation

## License

This project is licensed under the MIT License.
