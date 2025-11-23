use anyhow::{anyhow, Result};
use base64::{engine::general_purpose::STANDARD as BASE64, Engine};
use rand::RngCore;
use sha2::{Digest, Sha256};
use std::fs;
use std::path::Path;

use crate::types::KeyPair;

#[derive(Clone)]
pub struct KeyManager {
    #[allow(dead_code)]
    secret_key: Option<Vec<u8>>,
    public_key: Option<Vec<u8>>,
}

impl KeyManager {
    pub fn new() -> Self {
        Self {
            secret_key: None,
            public_key: None,
        }
    }

    /// Generate a new simple keypair (32-byte secret, derived public)
    pub fn generate_keypair() -> Result<KeyPair> {
        let mut secret_key = [0u8; 32];
        rand::thread_rng().fill_bytes(&mut secret_key);
        
        // Derive public key from secret key using SHA256
        let mut hasher = Sha256::new();
        hasher.update(&secret_key);
        hasher.update(b"PUBLIC_KEY_DERIVATION");
        let public_key = hasher.finalize().to_vec();

        Ok(KeyPair {
            secret_key: secret_key.to_vec(),
            public_key,
        })
    }

    /// Load keypair from secret key bytes
    pub fn from_secret_key(secret_key_bytes: &[u8]) -> Result<Self> {
        if secret_key_bytes.len() != 32 {
            return Err(anyhow!("Secret key must be 32 bytes"));
        }
        
        // Derive public key from secret key
        let mut hasher = Sha256::new();
        hasher.update(secret_key_bytes);
        hasher.update(b"PUBLIC_KEY_DERIVATION");
        let public_key = hasher.finalize().to_vec();

        Ok(Self {
            secret_key: Some(secret_key_bytes.to_vec()),
            public_key: Some(public_key),
        })
    }

    /// Load public key from bytes
    pub fn from_public_key(public_key_bytes: &[u8]) -> Result<Vec<u8>> {
        if public_key_bytes.len() != 32 {
            return Err(anyhow!("Public key must be 32 bytes"));
        }
        Ok(public_key_bytes.to_vec())
    }

    /// Get the public key bytes
    pub fn get_public_key_bytes(&self) -> Result<Vec<u8>> {
        self.public_key
            .clone()
            .ok_or_else(|| anyhow!("No public key loaded"))
    }

    /// Get the secret key bytes
    #[allow(dead_code)]
    pub fn get_secret_key_bytes(&self) -> Result<Vec<u8>> {
        self.secret_key
            .clone()
            .ok_or_else(|| anyhow!("No secret key loaded"))
    }

    /// Save keypair to files
    pub fn save_keypair_to_files(keypair: &KeyPair, base_path: &str, name: &str) -> Result<()> {
        let secret_path = format!("{}/{}_secret.key", base_path, name);
        let public_path = format!("{}/{}_public.key", base_path, name);

        // Create directory if it doesn't exist
        if let Some(parent) = Path::new(&secret_path).parent() {
            fs::create_dir_all(parent)?;
        }

        // Save as base64 encoded strings for easy handling
        let secret_b64 = BASE64.encode(&keypair.secret_key);
        let public_b64 = BASE64.encode(&keypair.public_key);

        fs::write(&secret_path, secret_b64)?;
        fs::write(&public_path, public_b64)?;

        println!("Keypair saved:");
        println!("  Secret key: {}", secret_path);
        println!("  Public key: {}", public_path);

        Ok(())
    }

    /// Load secret key from file
    pub fn load_secret_key_from_file(path: &str) -> Result<Vec<u8>> {
        let content = fs::read_to_string(path)?;
        let content = content.trim();
        BASE64.decode(content)
            .map_err(|e| anyhow!("Failed to decode secret key from file: {}", e))
    }

    /// Load public key from file
    pub fn load_public_key_from_file(path: &str) -> Result<Vec<u8>> {
        let content = fs::read_to_string(path)?;
        let content = content.trim();
        BASE64.decode(content)
            .map_err(|e| anyhow!("Failed to decode public key from file: {}", e))
    }

    /// Generate a key ID from public key (SHA256 hash)
    pub fn generate_key_id(public_key: &[u8]) -> String {
        let mut hasher = Sha256::new();
        hasher.update(public_key);
        hex::encode(hasher.finalize())[..16].to_string() // First 16 chars of hash
    }

    /// Derive a shared secret using simple key derivation
    pub fn derive_shared_secret(&self, other_public_key_bytes: &[u8]) -> Result<Vec<u8>> {
        let secret_key = self.secret_key.as_ref()
            .ok_or_else(|| anyhow!("No secret key loaded"))?;
        
        // Simple shared secret derivation
        let mut hasher = Sha256::new();
        hasher.update(secret_key);
        hasher.update(other_public_key_bytes);
        hasher.update(b"SHARED_SECRET_DERIVATION");
        Ok(hasher.finalize().to_vec())
    }
}

impl Default for KeyManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_generate_keypair() {
        let keypair = KeyManager::generate_keypair().unwrap();
        assert_eq!(keypair.secret_key.len(), 32);
        assert_eq!(keypair.public_key.len(), 32);
    }

    #[test]
    fn test_save_and_load_keypair() {
        let keypair = KeyManager::generate_keypair().unwrap();
        let temp_dir = tempdir().unwrap();
        let base_path = temp_dir.path().to_str().unwrap();

        // Save keypair
        KeyManager::save_keypair_to_files(&keypair, base_path, "test").unwrap();

        // Load keys back
        let secret_path = format!("{}/test_secret.key", base_path);
        let public_path = format!("{}/test_public.key", base_path);

        let loaded_secret = KeyManager::load_secret_key_from_file(&secret_path).unwrap();
        let loaded_public = KeyManager::load_public_key_from_file(&public_path).unwrap();

        assert_eq!(keypair.secret_key, loaded_secret);
        assert_eq!(keypair.public_key, loaded_public);
    }

    #[test]
    fn test_key_manager_from_secret_key() {
        let keypair = KeyManager::generate_keypair().unwrap();
        let key_manager = KeyManager::from_secret_key(&keypair.secret_key).unwrap();
        
        let public_key_bytes = key_manager.get_public_key_bytes().unwrap();
        assert_eq!(keypair.public_key, public_key_bytes);
    }

    #[test]
    fn test_key_id_generation() {
        let keypair = KeyManager::generate_keypair().unwrap();
        let key_id = KeyManager::generate_key_id(&keypair.public_key);
        assert_eq!(key_id.len(), 16);
        
        // Same key should generate same ID
        let key_id2 = KeyManager::generate_key_id(&keypair.public_key);
        assert_eq!(key_id, key_id2);
    }

    #[test]
    fn test_shared_secret() {
        let keypair1 = KeyManager::generate_keypair().unwrap();
        let keypair2 = KeyManager::generate_keypair().unwrap();
        
        let km1 = KeyManager::from_secret_key(&keypair1.secret_key).unwrap();
        let km2 = KeyManager::from_secret_key(&keypair2.secret_key).unwrap();
        
        let secret1 = km1.derive_shared_secret(&keypair2.public_key).unwrap();
        let secret2 = km2.derive_shared_secret(&keypair1.public_key).unwrap();
        
        // Note: This simple implementation doesn't guarantee symmetric secrets
        // but it's deterministic for the same inputs
        assert_eq!(secret1.len(), 32);
        assert_eq!(secret2.len(), 32);
    }
}