import { join } from "node:path";
import { handleCameras } from "./routes/cameras";
import { handleConfig } from "./routes/config";
import { handleDiskClear, handleDiskInfo } from "./routes/disk";
import { handleImage } from "./routes/image";

const PORT = Number(process.env.PORT ?? 3000);
const DIST_DIR = join(import.meta.dir, "..", "dist");

const MIME: Record<string, string> = {
  ".html": "text/html; charset=utf-8",
  ".js": "application/javascript",
  ".mjs": "application/javascript",
  ".css": "text/css",
  ".png": "image/png",
  ".jpg": "image/jpeg",
  ".ico": "image/x-icon",
  ".svg": "image/svg+xml",
  ".wasm": "application/wasm",
};

function mimeFor(path: string): string {
  for (const [ext, type] of Object.entries(MIME)) {
    if (path.endsWith(ext)) return type;
  }
  return "application/octet-stream";
}

async function serveStatic(pathname: string): Promise<Response> {
  // SPA fallback: serve index.html for non-asset paths
  const isAsset = pathname.includes(".");
  const filePath = isAsset ? join(DIST_DIR, pathname) : join(DIST_DIR, "index.html");

  const file = Bun.file(filePath);
  if (!(await file.exists())) {
    const index = Bun.file(join(DIST_DIR, "index.html"));
    if (await index.exists()) {
      return new Response(index, { headers: { "Content-Type": "text/html; charset=utf-8" } });
    }
    return new Response("Not found", { status: 404 });
  }

  return new Response(file, {
    headers: { "Content-Type": mimeFor(filePath) },
  });
}

Bun.serve({
  port: PORT,
  async fetch(req: Request): Promise<Response> {
    const url = new URL(req.url);
    const path = url.pathname;

    // API routes
    if (path === "/api/cameras") return handleCameras(url);
    if (path === "/api/image") return handleImage(url);
    if (path === "/api/config") return handleConfig(req);
    if (path === "/api/disk/info") return handleDiskInfo(url);
    if (path === "/api/disk/clear") return handleDiskClear(req);

    // Static file serving (built frontend)
    return serveStatic(path);
  },
});

console.log(`Traffic Camera Viewer server running on http://localhost:${PORT}`);
