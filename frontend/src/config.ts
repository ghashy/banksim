import { env } from "./env";
export const API_URL = env.VITE_REACT_APP_API_URL || "://localhost:15100";
export const IS_SECURE = env.VITE_REACT_APP_IS_SECURE || "";

export const MAX_RETRIES = 7;
export const RETRY_DELAY_MS = 1000;

const username = "ghashy";
const password = "terminalpassword";
const token = btoa(`${username}:${password}`);
export const AUTH_HEADER = `Basic ${token}`;
