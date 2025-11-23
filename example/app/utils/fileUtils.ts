/**
 * Get MIME type based on file extension
 */
export function getMimeType(filename: string): string {
  const ext = filename.split('.').pop()?.toLowerCase();
  const mimeTypes: { [key: string]: string } = {
    'jpg': 'image/jpeg',
    'jpeg': 'image/jpeg',
    'png': 'image/png',
    'gif': 'image/gif',
    'webp': 'image/webp',
    'pdf': 'application/pdf',
    'txt': 'text/plain',
    'json': 'application/json',
    'csv': 'text/csv',
    'zip': 'application/zip',
    'mp4': 'video/mp4',
    'mp3': 'audio/mpeg',
  };
  return mimeTypes[ext || ''] || 'application/octet-stream';
}

/**
 * Download a file with the given content and filename
 */
export function downloadFile(content: Blob | string, filename: string, mimeType?: string): void {
  let blob: Blob;
  
  if (content instanceof Blob) {
    blob = content;
  } else {
    blob = new Blob([content], { type: mimeType || 'application/octet-stream' });
  }
  
  const url = URL.createObjectURL(blob);
  const a = document.createElement('a');
  a.href = url;
  a.download = filename;
  a.click();
  URL.revokeObjectURL(url);
}

/**
 * Convert base64 string to Uint8Array efficiently using fetch API
 */
export async function base64ToBytes(base64: string): Promise<Uint8Array> {
  const dataUrl = `data:application/octet-stream;base64,${base64}`;
  const response = await fetch(dataUrl);
  return new Uint8Array(await response.arrayBuffer());
}
