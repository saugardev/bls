"use client";

import { useState } from "react";
import { useBLSWasm } from "./useBLSWasm";
import { downloadFile, getMimeType, base64ToBytes } from "../utils/fileUtils";

interface KeyPair {
  secret_key: string;
  public_key: string;
  key_id: string;
}

interface EncryptedData {
  encrypted_data: string;
  data_hash: string;
  public_key_id: string;
  original_filename?: string;
  original_size?: number;
}

interface DecryptResult {
  success: boolean;
  data?: string;
  error?: string;
  hash_matches?: boolean;
}

export function useEncryption() {
  const { wasmModule, loading: wasmLoading, error: wasmError } = useBLSWasm();
  const [keyPair, setKeyPair] = useState<KeyPair | null>(null);
  const [selectedFile, setSelectedFile] = useState<File | null>(null);
  const [encryptedData, setEncryptedData] = useState<EncryptedData | null>(null);
  const [decryptedData, setDecryptedData] = useState<string | null>(null);
  const [providedKey, setProvidedKey] = useState<string>("");
  const [providedHash, setProvidedHash] = useState<string>("");
  const [hashValidation, setHashValidation] = useState<boolean | null>(null);
  const [loading, setLoading] = useState<boolean>(false);
  const [error, setError] = useState<string>("");

  const generateKeyPair = () => {
    if (!wasmModule) return;
    
    try {
      const bls = wasmModule.new();
      const keyPairJson = bls.generate_keypair();
      const keyPair = JSON.parse(keyPairJson);
      setKeyPair(keyPair);
      setError("");
    } catch (err) {
      setError("Failed to generate keypair: " + err);
    }
  };

  const handleFileSelect = (event: React.ChangeEvent<HTMLInputElement>) => {
    const file = event.target.files?.[0];
    if (file) {
      setSelectedFile(file);
      setError("");
    }
  };

  const encryptFile = async () => {
    if (!wasmModule || !selectedFile || !keyPair) return;
    
    setLoading(true);
    try {
      const bls = wasmModule.new();
      const fileData = new Uint8Array(await selectedFile.arrayBuffer());
      
      const encryptedJson = bls.encrypt(fileData, keyPair.public_key);
      const encrypted = JSON.parse(encryptedJson);
      
      // Add original filename and size to encrypted data
      encrypted.original_filename = selectedFile.name;
      encrypted.original_size = selectedFile.size;
      
      setEncryptedData(encrypted);
      setError("");
      
      // Download encrypted data
      downloadFile(
        JSON.stringify(encrypted, null, 2),
        `${selectedFile.name}.encrypted.json`,
        'application/json'
      );
      
    } catch (err) {
      setError("Encryption failed: " + err);
    } finally {
      setLoading(false);
    }
  };

  const handleEncryptedFileSelect = (event: React.ChangeEvent<HTMLInputElement>) => {
    const file = event.target.files?.[0];
    if (file) {
      const reader = new FileReader();
      reader.onload = (e) => {
        try {
          const encrypted = JSON.parse(e.target?.result as string);
          setEncryptedData(encrypted);
          setError("");
        } catch (err) {
          setError("Invalid encrypted file format");
        }
      };
      reader.readAsText(file);
    }
  };

  const decryptData = async () => {
    if (!wasmModule || !encryptedData || !providedKey) return;
    
    setLoading(true);
    try {
      const bls = wasmModule.new();
      const decryptResultJson = bls.decrypt(encryptedData.encrypted_data, providedKey);
      const result: DecryptResult = JSON.parse(decryptResultJson);
      
      if (result.success && result.data) {
        setDecryptedData(result.data);
        
        // Convert base64 to bytes efficiently
        const bytes = await base64ToBytes(result.data);
        
        // Validate hash if provided
        if (providedHash) {
          const isValid = bls.validate_hash(bytes, providedHash);
          setHashValidation(isValid);
        }
        
        // Determine filename and MIME type
        const originalFilename = encryptedData.original_filename || 'decrypted_file';
        const mimeType = getMimeType(originalFilename);
        
        // Download decrypted data with proper filename and type
        const blob = new Blob([bytes], { type: mimeType });
        downloadFile(blob, originalFilename);
        
        setError("");
      } else {
        setError(result.error || "Decryption failed");
      }
    } catch (err) {
      setError("Decryption failed: " + err);
    } finally {
      setLoading(false);
    }
  };

  return {
    // State
    keyPair,
    selectedFile,
    encryptedData,
    decryptedData,
    providedKey,
    providedHash,
    hashValidation,
    loading,
    error,
    wasmLoading,
    wasmError,
    wasmModule,
    
    // Actions
    generateKeyPair,
    handleFileSelect,
    encryptFile,
    handleEncryptedFileSelect,
    decryptData,
    setProvidedKey,
    setProvidedHash,
  };
}
