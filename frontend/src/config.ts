export const API_URL = window.location.origin.replace(/:\d+$/, ":15100");

export const MAX_RETRIES = 7;
export const RETRY_DELAY_MS = 1000;

const username = "ghashy";
const password = "terminalpassword";
const token = btoa(`${username}:${password}`);
export const AUTH_HEADER = `Basic ${token}`;
