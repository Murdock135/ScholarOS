import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";
export default defineConfig(({ mode }) => ({
  root: "apps/desktop",
  plugins: [react()],
  server: {
    host: "127.0.0.1",
    port: 1420,
    strictPort: true,
    proxy: mode === "test" ? { "/__test": "http://127.0.0.1:4319" } : undefined,
  },
  clearScreen: false,
}));
