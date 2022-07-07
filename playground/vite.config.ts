import { defineConfig } from "vite";
import wasm from "vite-plugin-wasm";
import react from "@vitejs/plugin-react";

// https://vitejs.dev/config/
export default defineConfig({
  server: {
    fs: {
      // Allow serving files from one level up to the project root
      allow: ["../pkg", "."],
    },
  },
  plugins: [
    react(),
    wasm({
      filter: /.+\.wasm$/,
    }),
  ],
});
