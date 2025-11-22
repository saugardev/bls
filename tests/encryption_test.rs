use bls_encryption_service::{
    encryption::BLSEncryption,
    keys::KeyManager,
};
use tempfile::tempdir;

#[test]
fn test_full_encryption_flow() {
    // Generate keypair
    let keypair = KeyManager::generate_keypair().unwrap();
    let encryption = BLSEncryption::new();

    // Test various data sizes
    let test_cases = vec![
        ("Empty data", vec![]),
        ("Small text", b"Hello, World!".to_vec()),
        ("Medium data", vec![0x42; 1024]),
        ("Large data", vec![0x55; 1024 * 1024]), // 1MB
    ];

    for (name, data) in test_cases {
        println!("Testing: {}", name);
        
        // Encrypt
        let encrypted = encryption.encrypt(&data, &keypair.public_key).unwrap();
        
        // Verify encrypted data structure
        assert!(!encrypted.encrypted_aes_key.is_empty());
        assert!(!encrypted.nonce.is_empty());
        assert!(!encrypted.data_hash.is_empty());
        assert!(!encrypted.public_key_id.is_empty());
        
        // Decrypt
        let decrypted = encryption.decrypt(&encrypted, &keypair.secret_key).unwrap();
        
        // Verify
        assert_eq!(data, decrypted, "Failed for test case: {}", name);
    }
}

#[test]
fn test_key_management() {
    let temp_dir = tempdir().unwrap();
    let base_path = temp_dir.path().to_str().unwrap();

    // Generate and save keypair
    let keypair = KeyManager::generate_keypair().unwrap();
    KeyManager::save_keypair_to_files(&keypair, base_path, "test").unwrap();

    // Load keys back
    let secret_path = format!("{}/test_secret.key", base_path);
    let public_path = format!("{}/test_public.key", base_path);

    let loaded_secret = KeyManager::load_secret_key_from_file(&secret_path).unwrap();
    let loaded_public = KeyManager::load_public_key_from_file(&public_path).unwrap();

    // Verify keys match
    assert_eq!(keypair.secret_key, loaded_secret);
    assert_eq!(keypair.public_key, loaded_public);

    // Test encryption with loaded keys
    let encryption = BLSEncryption::new();
    let test_data = b"Test with loaded keys";
    
    let encrypted = encryption.encrypt(test_data, &loaded_public).unwrap();
    let decrypted = encryption.decrypt(&encrypted, &loaded_secret).unwrap();
    
    assert_eq!(test_data.to_vec(), decrypted);
}

#[test]
fn test_text_encryption() {
    let keypair = KeyManager::generate_keypair().unwrap();
    let encryption = BLSEncryption::new();

    let long_text = "Very long text ".repeat(1000);
    let test_texts = vec![
        "",
        "Hello",
        "Multi\nline\ntext",
        "Unicode: 🔐🔑💻",
        &long_text,
    ];

    for text in test_texts {
        let encrypted_text = encryption.encrypt_text(text, &keypair.public_key).unwrap();
        let decrypted_text = encryption.decrypt_text(&encrypted_text, &keypair.secret_key).unwrap();
        assert_eq!(text, decrypted_text);
    }
}

#[test]
fn test_multiple_keypairs() {
    let encryption = BLSEncryption::new();
    let test_data = b"Cross-keypair test";

    // Generate multiple keypairs
    let keypair1 = KeyManager::generate_keypair().unwrap();
    let keypair2 = KeyManager::generate_keypair().unwrap();
    let keypair3 = KeyManager::generate_keypair().unwrap();

    // Encrypt with each public key
    let encrypted1 = encryption.encrypt(test_data, &keypair1.public_key).unwrap();
    let encrypted2 = encryption.encrypt(test_data, &keypair2.public_key).unwrap();
    let encrypted3 = encryption.encrypt(test_data, &keypair3.public_key).unwrap();

    // Decrypt with corresponding secret keys
    let decrypted1 = encryption.decrypt(&encrypted1, &keypair1.secret_key).unwrap();
    let decrypted2 = encryption.decrypt(&encrypted2, &keypair2.secret_key).unwrap();
    let decrypted3 = encryption.decrypt(&encrypted3, &keypair3.secret_key).unwrap();

    // All should match original data
    assert_eq!(test_data.to_vec(), decrypted1);
    assert_eq!(test_data.to_vec(), decrypted2);
    assert_eq!(test_data.to_vec(), decrypted3);

    // Cross-decryption should fail
    assert!(encryption.decrypt(&encrypted1, &keypair2.secret_key).is_err());
    assert!(encryption.decrypt(&encrypted2, &keypair3.secret_key).is_err());
    assert!(encryption.decrypt(&encrypted3, &keypair1.secret_key).is_err());
}

#[test]
fn test_data_integrity() {
    let keypair = KeyManager::generate_keypair().unwrap();
    let encryption = BLSEncryption::new();
    let test_data = b"Integrity test data";

    let mut encrypted = encryption.encrypt(test_data, &keypair.public_key).unwrap();

    // Test tampering with different parts
    let original_encrypted = encrypted.clone();

    // Tamper with encrypted content
    encrypted.encrypted_content[0] ^= 1;
    assert!(encryption.decrypt(&encrypted, &keypair.secret_key).is_err());
    encrypted = original_encrypted.clone();

    // Tamper with nonce
    encrypted.nonce[0] ^= 1;
    assert!(encryption.decrypt(&encrypted, &keypair.secret_key).is_err());
    encrypted = original_encrypted.clone();

    // Tamper with encrypted AES key
    encrypted.encrypted_aes_key[0] ^= 1;
    assert!(encryption.decrypt(&encrypted, &keypair.secret_key).is_err());

    // Original should still work
    let decrypted = encryption.decrypt(&original_encrypted, &keypair.secret_key).unwrap();
    assert_eq!(test_data.to_vec(), decrypted);
}

#[test]
fn test_key_id_consistency() {
    let keypair = KeyManager::generate_keypair().unwrap();
    let encryption = BLSEncryption::new();

    // Multiple encryptions with same key should have same key ID
    let encrypted1 = encryption.encrypt(b"data1", &keypair.public_key).unwrap();
    let encrypted2 = encryption.encrypt(b"data2", &keypair.public_key).unwrap();

    assert_eq!(encrypted1.public_key_id, encrypted2.public_key_id);

    // Different keys should have different IDs
    let keypair2 = KeyManager::generate_keypair().unwrap();
    let encrypted3 = encryption.encrypt(b"data3", &keypair2.public_key).unwrap();

    assert_ne!(encrypted1.public_key_id, encrypted3.public_key_id);
}

#[test]
fn test_hash_consistency() {
    let keypair = KeyManager::generate_keypair().unwrap();
    let encryption = BLSEncryption::new();

    let test_data = b"Hash test data";
    
    // Multiple encryptions of same data should have same hash
    let encrypted1 = encryption.encrypt(test_data, &keypair.public_key).unwrap();
    let encrypted2 = encryption.encrypt(test_data, &keypair.public_key).unwrap();

    assert_eq!(encrypted1.data_hash, encrypted2.data_hash);

    // Different data should have different hash
    let encrypted3 = encryption.encrypt(b"Different data", &keypair.public_key).unwrap();
    assert_ne!(encrypted1.data_hash, encrypted3.data_hash);

    // Hash should match direct computation
    let direct_hash = BLSEncryption::compute_hash(test_data);
    assert_eq!(encrypted1.data_hash, direct_hash);
}

#[test]
fn test_performance_requirements() {
    use std::time::Instant;

    // Test keypair generation speed (should be < 100ms)
    let start = Instant::now();
    let _keypair = KeyManager::generate_keypair().unwrap();
    let keygen_time = start.elapsed();
    assert!(keygen_time.as_millis() < 100, "Keypair generation too slow: {:?}", keygen_time);

    let keypair = KeyManager::generate_keypair().unwrap();
    let encryption = BLSEncryption::new();

    // Test 1MB encryption speed (should be < 500ms)
    let data_1mb = vec![0x42; 1024 * 1024];
    let start = Instant::now();
    let encrypted = encryption.encrypt(&data_1mb, &keypair.public_key).unwrap();
    let encrypt_time = start.elapsed();
    assert!(encrypt_time.as_millis() < 500, "1MB encryption too slow: {:?}", encrypt_time);

    // Test 1MB decryption speed (should be similar to encryption)
    let start = Instant::now();
    let _decrypted = encryption.decrypt(&encrypted, &keypair.secret_key).unwrap();
    let decrypt_time = start.elapsed();
    assert!(decrypt_time.as_millis() < 500, "1MB decryption too slow: {:?}", decrypt_time);
}

#[test]
fn test_large_file_simulation() {
    let keypair = KeyManager::generate_keypair().unwrap();
    let encryption = BLSEncryption::new();

    // Test with 10MB data (simulating large file)
    let large_data = vec![0x33; 10 * 1024 * 1024];
    
    let start = std::time::Instant::now();
    let encrypted = encryption.encrypt(&large_data, &keypair.public_key).unwrap();
    let encrypt_time = start.elapsed();
    
    let start = std::time::Instant::now();
    let decrypted = encryption.decrypt(&encrypted, &keypair.secret_key).unwrap();
    let decrypt_time = start.elapsed();

    assert_eq!(large_data, decrypted);
    
    println!("10MB encryption time: {:?}", encrypt_time);
    println!("10MB decryption time: {:?}", decrypt_time);
    
    // Should complete within reasonable time (10MB in <5 seconds)
    assert!(encrypt_time.as_secs() < 5);
    assert!(decrypt_time.as_secs() < 5);
}
