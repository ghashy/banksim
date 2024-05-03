import { env } from "./env";
export const API_URL = env.VITE_REACT_APP_API_URL || "http://localhost:15100";

export const MAX_RETRIES = 7;
export const RETRY_DELAY_MS = 1000;
