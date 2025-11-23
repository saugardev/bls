use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};
use base64::{engine::general_purpose::STANDARD as BASE64, Engine};

pub mod encryption;
pub mod keys;
pub mod types;

use encryption::BLSEncryption;
use keys::KeyManager;
use types::EncryptedData;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
    
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[derive(Serialize, Deserialize)]
pub struct WasmKeyPair {
    pub secret_key: String,  // base64 encoded
    pub public_key: String,  // base64 encoded
    pub key_id: String,
}

#[derive(Serialize, Deserialize)]
pub struct WasmEncryptedData {
    pub encrypted_data: String,  // base64 encoded JSON
    pub data_hash: String,
    pub public_key_id: String,
}

#[derive(Serialize, Deserialize)]
pub struct WasmDecryptResult {
    pub success: bool,
    pub data: Option<String>,  // base64 encoded
    pub error: Option<String>,
    pub hash_matches: Option<bool>,
}

#[wasm_bindgen]
pub struct BLSWasm {
    encryption: BLSEncryption,
}

#[wasm_bindgen]
impl BLSWasm {
    #[wasm_bindgen(constructor)]
    pub fn new() -> BLSWasm {
        console_error_panic_hook::set_once();
        BLSWasm {
            encryption: BLSEncryption::new(),
        }
    }

    /// Generate a new keypair
    #[wasm_bindgen]
    pub fn generate_keypair(&self) -> Result<String, JsValue> {
        let keypair = KeyManager::generate_keypair()
            .map_err(|e| JsValue::from_str(&format!("Failed to generate keypair: {}", e)))?;
        
        let key_id = KeyManager::generate_key_id(&keypair.public_key);
        
        let wasm_keypair = WasmKeyPair {
            secret_key: BASE64.encode(&keypair.secret_key),
            public_key: BASE64.encode(&keypair.public_key),
            key_id,
        };
        
        serde_json::to_string(&wasm_keypair)
            .map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))
    }

    /// Encrypt data with a public key
    #[wasm_bindgen]
    pub fn encrypt(&self, data: &[u8], public_key_b64: &str) -> Result<String, JsValue> {
        let public_key_bytes = BASE64.decode(public_key_b64)
            .map_err(|e| JsValue::from_str(&format!("Invalid public key: {}", e)))?;
        
        let encrypted_data = self.encryption.encrypt(data, &public_key_bytes)
            .map_err(|e| JsValue::from_str(&format!("Encryption failed: {}", e)))?;
        
        let serialized = serde_json::to_vec(&encrypted_data)
            .map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))?;
        
        let wasm_result = WasmEncryptedData {
            encrypted_data: BASE64.encode(serialized),
            data_hash: encrypted_data.data_hash.clone(),
            public_key_id: encrypted_data.public_key_id.clone(),
        };
        
        serde_json::to_string(&wasm_result)
            .map_err(|e| JsValue::from_str(&format!("Result serialization error: {}", e)))
    }

    /// Decrypt data with a secret key
    #[wasm_bindgen]
    pub fn decrypt(&self, encrypted_data_b64: &str, secret_key_b64: &str) -> Result<String, JsValue> {
        let secret_key_bytes = BASE64.decode(secret_key_b64)
            .map_err(|e| JsValue::from_str(&format!("Invalid secret key: {}", e)))?;
        
        let encrypted_data_bytes = BASE64.decode(encrypted_data_b64)
            .map_err(|e| JsValue::from_str(&format!("Invalid encrypted data: {}", e)))?;
        
        let encrypted_data: EncryptedData = serde_json::from_slice(&encrypted_data_bytes)
            .map_err(|e| JsValue::from_str(&format!("Failed to parse encrypted data: {}", e)))?;
        
        match self.encryption.decrypt(&encrypted_data, &secret_key_bytes) {
            Ok(decrypted_data) => {
                let encoded_data = BASE64.encode(&decrypted_data);
                
                let result = WasmDecryptResult {
                    success: true,
                    data: Some(encoded_data),
                    error: None,
                    hash_matches: Some(true), // If decrypt succeeds, hash already validated
                };
                serde_json::to_string(&result)
                    .map_err(|e| JsValue::from_str(&format!("Result serialization error: {}", e)))
            }
            Err(e) => {
                let result = WasmDecryptResult {
                    success: false,
                    data: None,
                    error: Some(e.to_string()),
                    hash_matches: Some(false),
                };
                serde_json::to_string(&result)
                    .map_err(|e| JsValue::from_str(&format!("Error serialization error: {}", e)))
            }
        }
    }

    /// Validate hash against decrypted data
    #[wasm_bindgen]
    pub fn validate_hash(&self, data: &[u8], expected_hash: &str) -> bool {
        let computed_hash = BLSEncryption::compute_hash(data);
        computed_hash == expected_hash
    }

    /// Compute SHA256 hash of data
    #[wasm_bindgen]
    pub fn compute_hash(&self, data: &[u8]) -> String {
        BLSEncryption::compute_hash(data)
    }

}

#[wasm_bindgen(start)]
pub fn main() {
    console_log!("BLS Encryption WASM module loaded");
}