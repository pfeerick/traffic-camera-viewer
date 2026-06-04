mod camera;
mod commands;
mod config;
mod error;
mod fetcher;

use fetcher::HttpClient;
use std::sync::Arc;
use tauri::Manager as _;
use tokio::sync::Mutex;

pub struct AppState {
    pub client: HttpClient,
    pub camera_cache: Arc<Mutex<Option<Vec<camera::CameraInfo>>>>,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_log::Builder::new().build())
        .manage(AppState {
            client: HttpClient::new(),
            camera_cache: Arc::new(Mutex::new(None)),
        })
        .setup(|app| {
            let state = app.state::<AppState>();
            let client = state.client.clone();
            let cache = Arc::clone(&state.camera_cache);
            tauri::async_runtime::spawn(async move {
                match fetcher::fetch_camera_list(&client).await {
                    Ok(cameras) => {
                        *cache.lock().await = Some(cameras);
                        log::debug!("Camera list prefetched");
                    }
                    Err(e) => log::warn!("Startup camera prefetch failed: {e}"),
                }
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::camera::get_camera_list,
            commands::camera::refresh_camera_list,
            commands::camera::fetch_image,
            commands::config::get_config,
            commands::config::save_config,
            commands::disk::clear_cache,
            commands::disk::get_cache_info,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
