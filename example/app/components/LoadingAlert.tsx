"use client";

interface LoadingAlertProps {
  loading: boolean;
}

export function LoadingAlert({ loading }: LoadingAlertProps) {
  if (!loading) return null;

  return (
    <div className="mb-6 p-4 bg-blue-100 border border-blue-400 text-blue-700 rounded-lg">
      Loading BLS encryption module...
    </div>
  );
}
