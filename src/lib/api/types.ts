export type TitleAlign = "left" | "center" | "right";

export interface CameraInfo {
  id: number;
  name: string;
  district: string;
  locality: string;
  image_url: string;
}

export interface AppConfig {
  selected_districts: string[];
  refresh_interval_secs: number;
  column_count: number;
  save_to_disk: boolean;
  save_path: string;
  max_snapshots: number;
  grid_spacing: number;
  show_camera_titles: boolean;
  camera_title_font_size: number;
  camera_title_rgb: [number, number, number];
  camera_title_align: TitleAlign;
  camera_aspect_ratio: [number, number];
  app_background_rgb: [number, number, number];
  hidden_camera_ids: number[];
  camera_order: number[];
}

export type ImageResult =
  | { type: "changed"; hash: number; jpeg_b64: string }
  | { type: "unchanged"; hash: number };

export interface CacheInfo {
  exists: boolean;
  file_count: number;
  total_bytes: number;
}

export interface FetchImageParams {
  camera_id: number;
  image_url: string;
  previous_hash: number | null;
  save_to_disk: boolean;
  save_path: string;
  max_snapshots: number;
}

export interface ApiClient {
  getConfig(): Promise<AppConfig>;
  saveConfig(config: AppConfig): Promise<void>;
  getCameraList(): Promise<CameraInfo[]>;
  refreshCameraList(): Promise<CameraInfo[]>;
  fetchImage(params: FetchImageParams): Promise<ImageResult>;
  clearCache(path: string): Promise<void>;
  getCacheInfo(path: string): Promise<CacheInfo>;
}
