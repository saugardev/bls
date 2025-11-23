"use client";

import { useEncryption } from "./hooks/useEncryption";
import { KeyGeneration } from "./components/KeyGeneration";
import { FileEncryption } from "./components/FileEncryption";
import { FileDecryption } from "./components/FileDecryption";
import { LoadingAlert } from "./components/LoadingAlert";
import { ErrorAlert } from "./components/ErrorAlert";

export default function Home() {
  const {
    keyPair,
    selectedFile,
    encryptedData,
    providedKey,
    providedHash,
    hashValidation,
    loading,
    error,
    wasmLoading,
    wasmError,
    wasmModule,
    generateKeyPair,
    handleFileSelect,
    encryptFile,
    handleEncryptedFileSelect,
    decryptData,
    setProvidedKey,
    setProvidedHash,
  } = useEncryption();

  const isDisabled = !wasmModule || wasmLoading;

  return (
    <div className="min-h-screen bg-gradient-to-br from-blue-50 to-indigo-100 dark:from-gray-900 dark:to-gray-800 p-4">
      <div className="max-w-4xl mx-auto">
        <div className="text-center mb-8">
          <h1 className="text-4xl font-bold text-gray-900 dark:text-white mb-2">
            BLS Encryption Demo
          </h1>
          <p className="text-gray-600 dark:text-gray-300">
            Encrypt and decrypt files using BLS cryptography with WebAssembly
          </p>
        </div>

        <LoadingAlert loading={wasmLoading} />
        <ErrorAlert error={error} wasmError={wasmError} />

        <div className="grid md:grid-cols-2 gap-6">
          <KeyGeneration
            keyPair={keyPair}
            onGenerateKeyPair={generateKeyPair}
            loading={loading}
            disabled={isDisabled}
          />

          <FileEncryption
            selectedFile={selectedFile}
            keyPair={keyPair}
            encryptedData={encryptedData}
            loading={loading}
            disabled={isDisabled}
            onFileSelect={handleFileSelect}
            onEncryptFile={encryptFile}
          />

          <FileDecryption
            encryptedData={encryptedData}
            providedKey={providedKey}
            providedHash={providedHash}
            hashValidation={hashValidation}
            loading={loading}
            disabled={isDisabled}
            onEncryptedFileSelect={handleEncryptedFileSelect}
            onProvidedKeyChange={setProvidedKey}
            onProvidedHashChange={setProvidedHash}
            onDecryptData={decryptData}
          />
        </div>
      </div>
    </div>
  );
}
