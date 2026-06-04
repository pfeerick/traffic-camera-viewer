use crate::config::{self, AppConfig};
use crate::error::AppError;
use tauri::{AppHandle, Manager};

#[tauri::command]
pub async fn get_config(app: AppHandle) -> Result<AppConfig, AppError> {
    let path = app
        .path()
        .app_data_dir()
        .map_err(|e| AppError::Config(e.to_string()))?
        .join("config.json");
    Ok(config::load_config(&path))
}

#[tauri::command]
pub async fn save_config(app: AppHandle, config: AppConfig) -> Result<(), AppError> {
    let path = app
        .path()
        .app_data_dir()
        .map_err(|e| AppError::Config(e.to_string()))?
        .join("config.json");
    config::save_config(&config, &path)
}
