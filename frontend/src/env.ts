declare global {
  interface Window {
    env: any;
  }
}

type EnvType = {
  VITE_REACT_APP_API_URL: string;
};

export const env: EnvType = { ...process.env, ...window.env };
