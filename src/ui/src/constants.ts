const hostname = window && window.location.origin;

export const BASE_URL =
  hostname.includes('localhost') || hostname.includes('192.168.1') ? `${hostname.split(':')[0]}:${hostname.split(':')[1]}:5000` : hostname;
