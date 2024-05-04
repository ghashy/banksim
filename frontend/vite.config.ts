import { defineConfig, loadEnv } from "vite";
import react from "@vitejs/plugin-react-swc";

// https://vitejs.dev/config/
// export default defineConfig({
//   plugins: [react()],
// });

export default defineConfig(({ mode }) => {
  const env = loadEnv(mode, process.cwd(), "");
  return {
    define: {
      "process.env.VITE_REACT_APP_API_URL": JSON.stringify(
        env.VITE_REACT_APP_API_URL
      ),
      "process.env.VITE_REACT_APP_IS_SECURE": JSON.stringify(
        env.VITE_REACT_APP_IS_SECURE
      ),
    },
    plugins: [react()],
  };
});
