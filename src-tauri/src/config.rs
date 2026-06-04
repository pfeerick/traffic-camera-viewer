use crate::error::AppError;
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

/// Horizontal alignment for camera title text above each tile.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum TitleAlign {
    #[default]
    Left,
    Center,
    Right,
}

const NATIVE_CAMERA_IMAGE_SIZE: [u16; 2] = [320, 256];

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct AppConfig {
    pub selected_districts: BTreeSet<String>,
    pub refresh_interval_secs: u32,
    pub column_count: usize,
    pub save_to_disk: bool,
    pub save_path: String,
    pub max_snapshots: usize,
    pub grid_spacing: f32,
    pub show_camera_titles: bool,
    pub camera_title_font_size: f32,
    pub camera_title_rgb: [u8; 3],
    pub camera_title_align: TitleAlign,
    pub camera_aspect_ratio: [u16; 2],
    pub app_background_rgb: [u8; 3],
    pub hidden_camera_ids: BTreeSet<u32>,
    pub camera_order: Vec<u32>,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            selected_districts: std::iter::once("Wide Bay/Burnett".to_string()).collect(),
            refresh_interval_secs: 60,
            column_count: 3,
            save_to_disk: false,
            save_path: default_save_path(),
            max_snapshots: 5,
            grid_spacing: 6.0,
            show_camera_titles: true,
            camera_title_font_size: 12.0,
            camera_title_rgb: [220, 220, 220],
            camera_title_align: TitleAlign::Left,
            camera_aspect_ratio: NATIVE_CAMERA_IMAGE_SIZE,
            app_background_rgb: [24, 24, 24],
            hidden_camera_ids: BTreeSet::new(),
            camera_order: Vec::new(),
        }
    }
}

fn default_save_path() -> String {
    let exe_dir = std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(std::path::Path::to_path_buf))
        .unwrap_or_else(|| PathBuf::from("."));
    exe_dir
        .join("Traffic Camera Footage")
        .to_string_lossy()
        .to_string()
}

pub fn load_config(path: &Path) -> AppConfig {
    // Try to migrate from legacy confy TOML location on first run.
    if !path.exists()
        && let Some(migrated) = try_migrate_legacy_config()
        && save_config(&migrated, path).is_ok()
    {
        log::info!("Migrated config from legacy TOML location");
        return migrated;
    }

    let Ok(data) = std::fs::read_to_string(path) else {
        return AppConfig::default();
    };
    match serde_json::from_str(&data) {
        Ok(cfg) => cfg,
        Err(e) => {
            log::warn!("Failed to parse config, using defaults: {e}");
            AppConfig::default()
        }
    }
}

pub fn save_config(cfg: &AppConfig, path: &Path) -> Result<(), AppError> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let json = serde_json::to_string_pretty(cfg)?;
    std::fs::write(path, json)?;
    Ok(())
}

/// Attempt to read and parse the legacy confy TOML config from %APPDATA%.
/// Returns `None` if not found or parse fails.
fn try_migrate_legacy_config() -> Option<AppConfig> {
    #[cfg(target_os = "windows")]
    let base = std::env::var("APPDATA").ok().map(PathBuf::from)?;
    #[cfg(not(target_os = "windows"))]
    let base = dirs_sys_maybe_home().map(|h| h.join(".config"))?;

    let legacy_path = base
        .join("traffic-camera-viewer")
        .join("config")
        .join("traffic-camera-viewer.toml");

    let data = std::fs::read_to_string(legacy_path).ok()?;
    // We no longer depend on the toml crate; try JSON fallback first, then skip.
    // If the file exists it's TOML, which serde_json cannot parse — return None
    // to fall back to defaults. A proper TOML migration would require the toml crate.
    let _ = data; // suppress unused warning
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[allow(clippy::float_cmp)]
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
    #[allow(clippy::float_cmp)]
    fn config_round_trips_via_json() {
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

        let json = serde_json::to_string(&cfg).expect("serialize failed");
        let restored: AppConfig = serde_json::from_str(&json).expect("deserialize failed");

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
