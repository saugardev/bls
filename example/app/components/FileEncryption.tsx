"use client";

import { useRef } from "react";
import { Upload, Lock } from "lucide-react";

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

interface FileEncryptionProps {
  selectedFile: File | null;
  keyPair: KeyPair | null;
  encryptedData: EncryptedData | null;
  loading: boolean;
  disabled: boolean;
  onFileSelect: (event: React.ChangeEvent<HTMLInputElement>) => void;
  onEncryptFile: () => void;
}

export function FileEncryption({
  selectedFile,
  keyPair,
  encryptedData,
  loading,
  disabled,
  onFileSelect,
  onEncryptFile,
}: FileEncryptionProps) {
  const fileInputRef = useRef<HTMLInputElement>(null);

  return (
    <div className="bg-white dark:bg-gray-800 rounded-lg shadow-lg p-6">
      <h2 className="text-xl font-semibold mb-4 flex items-center gap-2">
        <Lock className="w-5 h-5" />
        File Encryption
      </h2>
      
      <div className="space-y-4">
        <div>
          <input
            ref={fileInputRef}
            type="file"
            onChange={onFileSelect}
            className="hidden"
          />
          <button
            onClick={() => fileInputRef.current?.click()}
            className="w-full border-2 border-dashed border-gray-300 hover:border-gray-400 rounded-lg p-4 text-center transition-colors"
          >
            <Upload className="w-8 h-8 mx-auto mb-2 text-gray-400" />
            <span className="text-gray-600 dark:text-gray-300">
              {selectedFile ? selectedFile.name : "Select file to encrypt"}
            </span>
          </button>
        </div>
        
        <button
          onClick={onEncryptFile}
          disabled={!selectedFile || !keyPair || loading || disabled}
          className="w-full bg-green-600 hover:bg-green-700 disabled:bg-gray-400 text-white font-medium py-2 px-4 rounded-lg transition-colors"
        >
          {loading ? "Encrypting..." : "Encrypt File"}
        </button>
        
        {encryptedData && (
          <div className="mt-4 space-y-2">
            <div className="p-3 bg-green-50 border border-green-200 rounded">
              <p className="text-sm font-medium text-green-800">Encryption successful!</p>
              <p className="text-xs text-green-600">
                Original: {encryptedData.original_filename} ({encryptedData.original_size} bytes)
              </p>
              <p className="text-xs text-green-600">Hash: {encryptedData.data_hash}</p>
              <p className="text-xs text-green-600">Key ID: {encryptedData.public_key_id}</p>
            </div>
          </div>
        )}
      </div>
    </div>
  );
}
