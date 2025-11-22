use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptedData {
    pub encrypted_aes_key: Vec<u8>,  // BLS encrypted AES key
    pub encrypted_content: Vec<u8>,   // AES encrypted data
    pub nonce: Vec<u8>,              // AES nonce
    pub data_hash: String,           // SHA256 of original data
    pub public_key_id: String,       // Which key was used
}

#[derive(Debug, Clone)]
pub struct KeyPair {
    pub secret_key: Vec<u8>,
    pub public_key: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptRequest {
    pub data: String,  // base64 encoded data
    pub public_key: String,  // base64 encoded public key
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecryptRequest {
    pub encrypted_data: EncryptedData,
    pub secret_key: String,  // base64 encoded secret key
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptResponse {
    pub encrypted_data: EncryptedData,
    pub success: bool,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecryptResponse {
    pub data: String,  // base64 encoded decrypted data
    pub success: bool,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublicKeyResponse {
    pub public_key: String,  // base64 encoded
    pub key_id: String,
}

