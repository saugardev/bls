"use client";

import { useRef } from "react";
import { Unlock, Hash, CheckCircle, XCircle } from "lucide-react";

interface EncryptedData {
  encrypted_data: string;
  data_hash: string;
  public_key_id: string;
  original_filename?: string;
  original_size?: number;
}

interface FileDecryptionProps {
  encryptedData: EncryptedData | null;
  providedKey: string;
  providedHash: string;
  hashValidation: boolean | null;
  loading: boolean;
  disabled: boolean;
  onEncryptedFileSelect: (event: React.ChangeEvent<HTMLInputElement>) => void;
  onProvidedKeyChange: (value: string) => void;
  onProvidedHashChange: (value: string) => void;
  onDecryptData: () => void;
}

export function FileDecryption({
  encryptedData,
  providedKey,
  providedHash,
  hashValidation,
  loading,
  disabled,
  onEncryptedFileSelect,
  onProvidedKeyChange,
  onProvidedHashChange,
  onDecryptData,
}: FileDecryptionProps) {
  const encryptedFileInputRef = useRef<HTMLInputElement>(null);

  return (
    <div className="bg-white dark:bg-gray-800 rounded-lg shadow-lg p-6 md:col-span-2">
      <h2 className="text-xl font-semibold mb-4 flex items-center gap-2">
        <Unlock className="w-5 h-5" />
        File Decryption
      </h2>
      
      <div className="grid md:grid-cols-2 gap-4">
        <div className="space-y-4">
          <div>
            <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
              Load Encrypted File
            </label>
            <input
              ref={encryptedFileInputRef}
              type="file"
              accept=".json"
              onChange={onEncryptedFileSelect}
              className="w-full p-2 border border-gray-300 rounded"
            />
            {encryptedData?.original_filename && (
              <p className="text-xs text-gray-600 mt-1">
                Original file: {encryptedData.original_filename} ({encryptedData.original_size} bytes)
              </p>
            )}
          </div>
          
          <div>
            <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
              Secret Key
            </label>
            <textarea
              value={providedKey}
              onChange={(e) => onProvidedKeyChange(e.target.value)}
              placeholder="Paste secret key here..."
              rows={3}
              className="w-full p-2 border border-gray-300 rounded text-sm font-mono"
            />
          </div>
        </div>
        
        <div className="space-y-4">
          <div>
            <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1 flex items-center gap-2">
              <Hash className="w-4 h-4" />
              Expected Hash (optional)
            </label>
            <input
              type="text"
              value={providedHash}
              onChange={(e) => onProvidedHashChange(e.target.value)}
              placeholder="Enter expected hash for validation..."
              className="w-full p-2 border border-gray-300 rounded text-sm font-mono"
            />
          </div>
          
          <button
            onClick={onDecryptData}
            disabled={!encryptedData || !providedKey || loading || disabled}
            className="w-full bg-purple-600 hover:bg-purple-700 disabled:bg-gray-400 text-white font-medium py-2 px-4 rounded-lg transition-colors"
          >
            {loading ? "Decrypting..." : "Decrypt File"}
          </button>
          
          {hashValidation !== null && (
            <div className={`p-3 rounded flex items-center gap-2 ${
              hashValidation 
                ? 'bg-green-50 border border-green-200 text-green-800' 
                : 'bg-red-50 border border-red-200 text-red-800'
            }`}>
              {hashValidation ? (
                <CheckCircle className="w-5 h-5" />
              ) : (
                <XCircle className="w-5 h-5" />
              )}
              <span className="text-sm font-medium">
                Hash validation: {hashValidation ? 'PASSED' : 'FAILED'}
              </span>
            </div>
          )}
        </div>
      </div>
    </div>
  );
}
