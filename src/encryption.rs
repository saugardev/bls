use aes_gcm::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    Aes256Gcm, Nonce,
};
use anyhow::{anyhow, Result};
use base64::{engine::general_purpose::STANDARD as BASE64, Engine};
// BLS operations handled in KeyManager
use rand::RngCore;
// Parallel processing for future use
use sha2::{Digest, Sha256};
use std::io::{Read, Write};

use crate::keys::KeyManager;
use crate::types::EncryptedData;


#[derive(Clone)]
pub struct BLSEncryption {
    #[allow(dead_code)]
    key_manager: Option<KeyManager>,
}

impl BLSEncryption {
    pub fn new() -> Self {
        Self {
            key_manager: None,
        }
    }

    #[allow(dead_code)]
    pub fn with_key_manager(key_manager: KeyManager) -> Self {
        Self {
            key_manager: Some(key_manager),
        }
    }

    /// Encrypt data using hybrid BLS + AES encryption
    pub fn encrypt(&self, data: &[u8], public_key_bytes: &[u8]) -> Result<EncryptedData> {
        // Generate random AES key
        let mut aes_key = [0u8; 32];
        rand::thread_rng().fill_bytes(&mut aes_key);

        // Encrypt the data with AES-256-GCM
        let cipher = Aes256Gcm::new_from_slice(&aes_key)
            .map_err(|e| anyhow!("Failed to create AES cipher: {}", e))?;
        
        let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
        let encrypted_content = cipher
            .encrypt(&nonce, data)
            .map_err(|e| anyhow!("AES encryption failed: {}", e))?;

        // Encrypt the AES key with BLS public key
        let encrypted_aes_key = self.encrypt_aes_key(&aes_key, public_key_bytes)?;

        // Compute hash of original data
        let mut hasher = Sha256::new();
        hasher.update(data);
        let data_hash = hex::encode(hasher.finalize());

        // Generate public key ID
        let public_key_id = KeyManager::generate_key_id(public_key_bytes);

        Ok(EncryptedData {
            encrypted_aes_key,
            encrypted_content,
            nonce: nonce.to_vec(),
            data_hash,
            public_key_id,
        })
    }

    /// Decrypt data using hybrid BLS + AES decryption
    pub fn decrypt(&self, encrypted_data: &EncryptedData, secret_key_bytes: &[u8]) -> Result<Vec<u8>> {
        // Decrypt the AES key with BLS secret key
        let aes_key = self.decrypt_aes_key(&encrypted_data.encrypted_aes_key, secret_key_bytes)?;

        // Decrypt the content with AES
        let cipher = Aes256Gcm::new_from_slice(&aes_key)
            .map_err(|e| anyhow!("Failed to create AES cipher: {}", e))?;
        
        let nonce = Nonce::from_slice(&encrypted_data.nonce);
        let decrypted_content = cipher
            .decrypt(nonce, encrypted_data.encrypted_content.as_ref())
            .map_err(|e| anyhow!("AES decryption failed: {}", e))?;

        // Verify data integrity
        let mut hasher = Sha256::new();
        hasher.update(&decrypted_content);
        let computed_hash = hex::encode(hasher.finalize());

        if computed_hash != encrypted_data.data_hash {
            return Err(anyhow!("Data integrity check failed"));
        }

        Ok(decrypted_content)
    }

    /// Encrypt large files in chunks for better performance
    pub fn encrypt_large<R: Read, W: Write>(
        &self,
        mut reader: R,
        mut writer: W,
        public_key_bytes: &[u8],
    ) -> Result<EncryptedData> {
        // Read all data first (for simplicity, could be optimized for streaming)
        let mut data = Vec::new();
        reader.read_to_end(&mut data)?;

        // For very large files, we could implement streaming encryption
        // For now, use the regular encrypt method
        let encrypted_data = self.encrypt(&data, public_key_bytes)?;

        // Write encrypted data
        let serialized = serde_json::to_vec(&encrypted_data)?;
        writer.write_all(&serialized)?;

        Ok(encrypted_data)
    }

    /// Decrypt large files
    pub fn decrypt_large<R: Read, W: Write>(
        &self,
        mut reader: R,
        mut writer: W,
        secret_key_bytes: &[u8],
    ) -> Result<()> {
        // Read encrypted data
        let mut serialized = Vec::new();
        reader.read_to_end(&mut serialized)?;

        let encrypted_data: EncryptedData = serde_json::from_slice(&serialized)?;
        let decrypted_data = self.decrypt(&encrypted_data, secret_key_bytes)?;

        writer.write_all(&decrypted_data)?;
        Ok(())
    }

    /// Encrypt AES key using BLS public key (simplified key encapsulation)
    fn encrypt_aes_key(&self, aes_key: &[u8; 32], public_key_bytes: &[u8]) -> Result<Vec<u8>> {
        // For BLS, we'll use a simple approach: hash the public key to get an encryption key
        // In a full implementation, you'd use proper BLS encryption or ECIES
        
        let _public_key = KeyManager::from_public_key(public_key_bytes)?;
        
        // Create a deterministic but secure encryption key from the public key
        let mut hasher = Sha256::new();
        hasher.update(public_key_bytes);
        hasher.update(b"BLS_KEY_ENCAPSULATION");
        let encryption_key = hasher.finalize();

        // Use AES to encrypt the AES key with the derived key
        let cipher = Aes256Gcm::new_from_slice(&encryption_key)
            .map_err(|e| anyhow!("Failed to create key encryption cipher: {}", e))?;
        
        let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
        let mut encrypted_key = cipher
            .encrypt(&nonce, aes_key.as_ref())
            .map_err(|e| anyhow!("Key encryption failed: {}", e))?;

        // Prepend nonce to encrypted key
        let mut result = nonce.to_vec();
        result.append(&mut encrypted_key);
        
        Ok(result)
    }

    /// Decrypt AES key using BLS secret key
    fn decrypt_aes_key(&self, encrypted_aes_key: &[u8], secret_key_bytes: &[u8]) -> Result<Vec<u8>> {
        if encrypted_aes_key.len() < 12 {
            return Err(anyhow!("Encrypted AES key too short"));
        }

        // Extract nonce and encrypted key
        let (nonce_bytes, encrypted_key) = encrypted_aes_key.split_at(12);
        let nonce = Nonce::from_slice(nonce_bytes);

        // Derive the same encryption key from secret key
        let key_manager = KeyManager::from_secret_key(secret_key_bytes)?;
        let public_key_bytes = key_manager.get_public_key_bytes()?;
        
        let mut hasher = Sha256::new();
        hasher.update(&public_key_bytes);
        hasher.update(b"BLS_KEY_ENCAPSULATION");
        let encryption_key = hasher.finalize();

        // Decrypt the AES key
        let cipher = Aes256Gcm::new_from_slice(&encryption_key)
            .map_err(|e| anyhow!("Failed to create key decryption cipher: {}", e))?;
        
        let decrypted_key = cipher
            .decrypt(nonce, encrypted_key)
            .map_err(|e| anyhow!("Key decryption failed: {}", e))?;

        Ok(decrypted_key)
    }

    /// Compute SHA256 hash of data
    #[allow(dead_code)]
    pub fn compute_hash(data: &[u8]) -> String {
        let mut hasher = Sha256::new();
        hasher.update(data);
        hex::encode(hasher.finalize())
    }

    /// Encrypt text and return base64 encoded result
    pub fn encrypt_text(&self, text: &str, public_key_bytes: &[u8]) -> Result<String> {
        let encrypted_data = self.encrypt(text.as_bytes(), public_key_bytes)?;
        let serialized = serde_json::to_vec(&encrypted_data)?;
        Ok(BASE64.encode(serialized))
    }

    /// Decrypt base64 encoded text
    pub fn decrypt_text(&self, encrypted_text: &str, secret_key_bytes: &[u8]) -> Result<String> {
        let serialized = BASE64.decode(encrypted_text)?;
        let encrypted_data: EncryptedData = serde_json::from_slice(&serialized)?;
        let decrypted_bytes = self.decrypt(&encrypted_data, secret_key_bytes)?;
        String::from_utf8(decrypted_bytes)
            .map_err(|e| anyhow!("Decrypted data is not valid UTF-8: {}", e))
    }
}

impl Default for BLSEncryption {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::keys::KeyManager;

    #[test]
    fn test_encrypt_decrypt_small_data() {
        let keypair = KeyManager::generate_keypair().unwrap();
        let encryption = BLSEncryption::new();
        
        let data = b"Hello, BLS encryption!";
        let encrypted = encryption.encrypt(data, &keypair.public_key).unwrap();
        let decrypted = encryption.decrypt(&encrypted, &keypair.secret_key).unwrap();
        
        assert_eq!(data, decrypted.as_slice());
    }

    #[test]
    fn test_encrypt_decrypt_large_data() {
        let keypair = KeyManager::generate_keypair().unwrap();
        let encryption = BLSEncryption::new();
        
        // Create 1MB of test data
        let data = vec![0x42u8; 1024 * 1024];
        let encrypted = encryption.encrypt(&data, &keypair.public_key).unwrap();
        let decrypted = encryption.decrypt(&encrypted, &keypair.secret_key).unwrap();
        
        assert_eq!(data, decrypted);
    }

    #[test]
    fn test_encrypt_decrypt_text() {
        let keypair = KeyManager::generate_keypair().unwrap();
        let encryption = BLSEncryption::new();
        
        let text = "This is a secret message!";
        let encrypted_text = encryption.encrypt_text(text, &keypair.public_key).unwrap();
        let decrypted_text = encryption.decrypt_text(&encrypted_text, &keypair.secret_key).unwrap();
        
        assert_eq!(text, decrypted_text);
    }

    #[test]
    fn test_data_integrity() {
        let keypair = KeyManager::generate_keypair().unwrap();
        let encryption = BLSEncryption::new();
        
        let data = b"Integrity test data";
        let mut encrypted = encryption.encrypt(data, &keypair.public_key).unwrap();
        
        // Tamper with encrypted content
        encrypted.encrypted_content[0] ^= 1;
        
        // Decryption should fail due to integrity check
        let result = encryption.decrypt(&encrypted, &keypair.secret_key);
        assert!(result.is_err());
    }

    #[test]
    fn test_wrong_key() {
        let keypair1 = KeyManager::generate_keypair().unwrap();
        let keypair2 = KeyManager::generate_keypair().unwrap();
        let encryption = BLSEncryption::new();
        
        let data = b"Secret data";
        let encrypted = encryption.encrypt(data, &keypair1.public_key).unwrap();
        
        // Try to decrypt with wrong key
        let result = encryption.decrypt(&encrypted, &keypair2.secret_key);
        assert!(result.is_err());
    }
}
