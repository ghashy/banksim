// export const API_URL = "://localhost:15100";
// export const IS_SECURE = "";

export const API_URL: string = (window as any).domain_name;
export const IS_SECURE: boolean = !!(window as any).secure;

export const MAX_RETRIES = 7;
export const RETRY_DELAY_MS = 1000;

const username = "ghashy";
const password = "terminalpassword";
const token = btoa(`${username}:${password}`);
export const AUTH_HEADER = `Basic ${token}`;
