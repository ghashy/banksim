declare global {
  interface Window {
    env: any;
  }
}

type EnvType = {
  VITE_REACT_APP_API_URL: string;
  VITE_REACT_APP_IS_SECURE: string;
};

export const env: EnvType = { ...process.env, ...window.env };
