import { tauriApi } from "./tauri";
import { webApi } from "./web";

export type {
  ApiClient,
  AppConfig,
  CacheInfo,
  CameraInfo,
  FetchImageParams,
  ImageResult,
  TitleAlign,
} from "./types";

export const isTauri = typeof window !== "undefined" && "__TAURI_INTERNALS__" in window;

export const api = isTauri ? tauriApi : webApi;
