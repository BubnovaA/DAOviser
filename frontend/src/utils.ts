// src/utils.ts 
export function convertIpfsUrl(url: string): string {
    if (url.startsWith('ipfs://')) {
      return url.replace('ipfs://', 'https://ipfs.io/ipfs/');
    }
    return url;
  }