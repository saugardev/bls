"use client";

interface ErrorAlertProps {
  error: string | null;
  wasmError: string | null;
}

export function ErrorAlert({ error, wasmError }: ErrorAlertProps) {
  if (!error && !wasmError) return null;

  return (
    <div className="mb-6 p-4 bg-red-100 border border-red-400 text-red-700 rounded-lg">
      {error || wasmError}
    </div>
  );
}
