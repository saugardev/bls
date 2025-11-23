"use client";

import { Key } from "lucide-react";

interface KeyPair {
  secret_key: string;
  public_key: string;
  key_id: string;
}

interface KeyGenerationProps {
  keyPair: KeyPair | null;
  onGenerateKeyPair: () => void;
  loading: boolean;
  disabled: boolean;
}

export function KeyGeneration({ keyPair, onGenerateKeyPair, loading, disabled }: KeyGenerationProps) {
  return (
    <div className="bg-white dark:bg-gray-800 rounded-lg shadow-lg p-6">
      <h2 className="text-xl font-semibold mb-4 flex items-center gap-2">
        <Key className="w-5 h-5" />
        Key Generation
      </h2>
      
      <button
        onClick={onGenerateKeyPair}
        disabled={disabled || loading}
        className="w-full bg-blue-600 hover:bg-blue-700 disabled:bg-gray-400 text-white font-medium py-2 px-4 rounded-lg transition-colors"
      >
        Generate New Keypair
      </button>
      
      {keyPair && (
        <div className="mt-4 space-y-3">
          <div>
            <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
              Key ID
            </label>
            <input
              type="text"
              value={keyPair.key_id}
              readOnly
              className="w-full p-2 border border-gray-300 rounded bg-gray-50 text-sm font-mono"
            />
          </div>
          <div>
            <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
              Public Key (for encryption)
            </label>
            <textarea
              value={keyPair.public_key}
              readOnly
              rows={3}
              className="w-full p-2 border border-gray-300 rounded bg-gray-50 text-sm font-mono"
            />
          </div>
          <div>
            <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
              Secret Key (keep private!)
            </label>
            <textarea
              value={keyPair.secret_key}
              readOnly
              rows={3}
              className="w-full p-2 border border-gray-300 rounded bg-red-50 text-sm font-mono"
            />
          </div>
        </div>
      )}
    </div>
  );
}
