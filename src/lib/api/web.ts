import type {
  ApiClient,
  AppConfig,
  CacheInfo,
  CameraInfo,
  FetchImageParams,
  ImageResult,
} from "./types";

const BASE = "/api";

async function json<T>(res: Response): Promise<T> {
  if (!res.ok) throw new Error(`HTTP ${res.status}: ${await res.text()}`);
  return res.json() as Promise<T>;
}

export const webApi: ApiClient = {
  getConfig: () => fetch(`${BASE}/config`).then(json<AppConfig>),

  saveConfig: (config: AppConfig) =>
    fetch(`${BASE}/config`, {
      method: "POST",
      body: JSON.stringify(config),
      headers: { "Content-Type": "application/json" },
    }).then(json<void>),

  getCameraList: () => fetch(`${BASE}/cameras`).then(json<CameraInfo[]>),

  refreshCameraList: () => fetch(`${BASE}/cameras?refresh=1`).then(json<CameraInfo[]>),

  fetchImage: (params: FetchImageParams) => {
    const q = new URLSearchParams({
      camera_id: String(params.camera_id),
      image_url: params.image_url,
      save_to_disk: String(params.save_to_disk),
      save_path: params.save_path,
      max_snapshots: String(params.max_snapshots),
    });
    if (params.previous_hash !== null) {
      q.set("previous_hash", String(params.previous_hash));
    }
    return fetch(`${BASE}/image?${q}`).then(json<ImageResult>);
  },

  clearCache: (path: string) =>
    fetch(`${BASE}/disk/clear`, {
      method: "POST",
      body: JSON.stringify({ path }),
      headers: { "Content-Type": "application/json" },
    }).then(json<void>),

  getCacheInfo: (path: string) =>
    fetch(`${BASE}/disk/info?path=${encodeURIComponent(path)}`).then(json<CacheInfo>),
};
