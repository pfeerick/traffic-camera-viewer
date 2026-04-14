use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use std::path::PathBuf;

const APP_NAME: &str = "traffic-camera-viewer";
const NATIVE_CAMERA_IMAGE_SIZE: [u32; 2] = [320, 256];

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct AppConfig {
    /// Districts whose cameras are shown.
    pub selected_districts: BTreeSet<String>,
    /// Auto-refresh period in seconds (10–300).
    pub refresh_interval_secs: u64,
    /// Number of columns in the image grid (1–6).
    pub column_count: usize,
    /// Save fetched images to disk.
    pub save_to_disk: bool,
    /// Directory to save images into.
    pub save_path: String,
    /// Rolling snapshot count per camera (1–20).
    pub max_snapshots: usize,
    /// Horizontal/vertical spacing between grid cells in points.
    pub grid_spacing: f32,
    /// Show camera titles above each image tile.
    pub show_camera_titles: bool,
    /// Font size for camera title text in points.
    pub camera_title_font_size: f32,
    /// Camera title text color as RGB.
    pub camera_title_rgb: [u8; 3],
    /// Camera image tile aspect ratio as width:height.
    pub camera_aspect_ratio: [u32; 2],
    /// Grid/background color as RGB (does not affect status bar panel).
    pub app_background_rgb: [u8; 3],
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            selected_districts: ["Wide Bay/Burnett".to_string()].into_iter().collect(),
            refresh_interval_secs: 60,
            column_count: 3,
            save_to_disk: false,
            save_path: default_save_path(),
            max_snapshots: 5,
            grid_spacing: 6.0,
            show_camera_titles: true,
            camera_title_font_size: 12.0,
            camera_title_rgb: [220, 220, 220],
            camera_aspect_ratio: NATIVE_CAMERA_IMAGE_SIZE,
            app_background_rgb: [24, 24, 24],
        }
    }
}

fn default_save_path() -> String {
    let exe_dir = std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|p| p.to_path_buf()))
        .unwrap_or_else(|| PathBuf::from("."));
    exe_dir
        .join("Traffic Camera Footage")
        .to_string_lossy()
        .to_string()
}

pub fn load_config() -> AppConfig {
    confy::load(APP_NAME, None).unwrap_or_default()
}

pub fn save_config(cfg: &AppConfig) {
    if let Err(e) = confy::store(APP_NAME, None, cfg) {
        log::warn!("Failed to save config: {e}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_config_has_wide_bay() {
        let cfg = AppConfig::default();
        assert!(cfg.selected_districts.contains("Wide Bay/Burnett"));
        assert_eq!(cfg.refresh_interval_secs, 60);
        assert_eq!(cfg.column_count, 3);
        assert!(!cfg.save_to_disk);
        assert_eq!(cfg.max_snapshots, 5);
        assert_eq!(cfg.grid_spacing, 6.0);
        assert!(cfg.show_camera_titles);
        assert_eq!(cfg.camera_title_font_size, 12.0);
        assert_eq!(cfg.camera_title_rgb, [220, 220, 220]);
        assert_eq!(cfg.camera_aspect_ratio, NATIVE_CAMERA_IMAGE_SIZE);
        assert_eq!(cfg.app_background_rgb, [24, 24, 24]);
    }

    #[test]
    fn config_round_trips_via_toml() {
        let mut cfg = AppConfig::default();
        cfg.selected_districts.insert("Metropolitan".to_string());
        cfg.refresh_interval_secs = 30;
        cfg.column_count = 4;
        cfg.save_to_disk = true;
        cfg.max_snapshots = 10;
        cfg.grid_spacing = 5.0;
        cfg.show_camera_titles = false;
        cfg.camera_title_font_size = 14.0;
        cfg.camera_title_rgb = [200, 180, 160];
        cfg.camera_aspect_ratio = [16, 9];
        cfg.app_background_rgb = [10, 20, 30];

        let toml_str = toml::to_string(&cfg).expect("serialize failed");
        let restored: AppConfig = toml::from_str(&toml_str).expect("deserialize failed");

        assert_eq!(restored.selected_districts, cfg.selected_districts);
        assert_eq!(restored.refresh_interval_secs, 30);
        assert_eq!(restored.column_count, 4);
        assert!(restored.save_to_disk);
        assert_eq!(restored.max_snapshots, 10);
        assert_eq!(restored.grid_spacing, 5.0);
        assert!(!restored.show_camera_titles);
        assert_eq!(restored.camera_title_font_size, 14.0);
        assert_eq!(restored.camera_title_rgb, [200, 180, 160]);
        assert_eq!(restored.camera_aspect_ratio, [16, 9]);
        assert_eq!(restored.app_background_rgb, [10, 20, 30]);
    }
}
