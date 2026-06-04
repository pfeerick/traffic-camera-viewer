import { join } from "node:path";
import { mkdir, readFile, writeFile } from "node:fs/promises";
import { homedir } from "node:os";

const CONFIG_DIR =
  process.env.XDG_CONFIG_HOME
    ? join(process.env.XDG_CONFIG_HOME, "traffic-camera-viewer")
    : join(homedir(), ".config", "traffic-camera-viewer");

const CONFIG_PATH = join(CONFIG_DIR, "config.json");

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
  camera_title_align: "left" | "center" | "right";
  camera_aspect_ratio: [number, number];
  app_background_rgb: [number, number, number];
  hidden_camera_ids: number[];
  camera_order: number[];
}

const DEFAULT_CONFIG: AppConfig = {
  selected_districts: ["Wide Bay/Burnett"],
  refresh_interval_secs: 60,
  column_count: 3,
  save_to_disk: false,
  save_path: "Traffic Camera Footage",
  max_snapshots: 5,
  grid_spacing: 6,
  show_camera_titles: true,
  camera_title_font_size: 12,
  camera_title_rgb: [220, 220, 220],
  camera_title_align: "left",
  camera_aspect_ratio: [320, 256],
  app_background_rgb: [24, 24, 24],
  hidden_camera_ids: [],
  camera_order: [],
};

export async function loadConfig(): Promise<AppConfig> {
  try {
    const data = await readFile(CONFIG_PATH, "utf-8");
    return { ...DEFAULT_CONFIG, ...(JSON.parse(data) as Partial<AppConfig>) };
  } catch {
    return { ...DEFAULT_CONFIG };
  }
}

export async function saveConfig(cfg: AppConfig): Promise<void> {
  await mkdir(CONFIG_DIR, { recursive: true });
  await writeFile(CONFIG_PATH, JSON.stringify(cfg, null, 2), "utf-8");
}
