import { defineConfig } from "vite";
import { svelte } from "@sveltejs/vite-plugin-svelte";
import path from "path";

const host = process.env.TAURI_DEV_HOST;

export default defineConfig({
  plugins: [svelte()],
  resolve: {
    alias: {
      $lib: path.resolve("./src/lib"),
    },
  },
  clearScreen: false,
  server: {
    port: 5173,
    strictPort: true,
    host: host || false,
    // Web mode talks to the Bun server over fetch('/api/...'). Without this proxy,
    // Vite answers those with the SPA fallback and the frontend fails to parse
    // '<!doctype html' as JSON. Harmless under `tauri:dev`, which uses invoke()
    // and never issues these requests. Needs `bun run server` running alongside.
    proxy: {
      "/api": {
        target: "http://localhost:3000",
        changeOrigin: true,
      },
    },
    hmr: host
      ? {
          protocol: "ws",
          host,
          port: 5183,
        }
      : undefined,
  },
  envPrefix: ["VITE_", "TAURI_ENV_*"],
  build: {
    target:
      process.env.TAURI_ENV_PLATFORM === "windows"
        ? "chrome105"
        : "safari13",
    minify: !process.env.TAURI_ENV_DEBUG,
    sourcemap: !!process.env.TAURI_ENV_DEBUG,
    outDir: "dist",
  },
});
