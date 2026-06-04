import { invoke } from "@tauri-apps/api/core";
import type {
  ApiClient,
  AppConfig,
  CacheInfo,
  CameraInfo,
  FetchImageParams,
  ImageResult,
} from "./types";

export const tauriApi: ApiClient = {
  getConfig: () => invoke<AppConfig>("get_config"),

  saveConfig: (config: AppConfig) => invoke<void>("save_config", { config }),

  getCameraList: () => invoke<CameraInfo[]>("get_camera_list"),

  refreshCameraList: () => invoke<CameraInfo[]>("refresh_camera_list"),

  fetchImage: (params: FetchImageParams) =>
    invoke<ImageResult>("fetch_image", {
      cameraId: params.camera_id,
      imageUrl: params.image_url,
      previousHash: params.previous_hash,
      saveToDisk: params.save_to_disk,
      savePath: params.save_path,
      maxSnapshots: params.max_snapshots,
    }),

  clearCache: (path: string) => invoke<void>("clear_cache", { path }),

  getCacheInfo: (path: string) => invoke<CacheInfo>("get_cache_info", { path }),
};
