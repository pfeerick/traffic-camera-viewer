use crate::AppState;
use crate::camera::CameraInfo;
use crate::error::AppError;
use crate::fetcher::{self, CameraImageFetchOutcome};
use tauri::State;

#[tauri::command]
pub async fn get_camera_list(state: State<'_, AppState>) -> Result<Vec<CameraInfo>, AppError> {
    let cached = state.camera_cache.lock().await.clone();
    if let Some(cameras) = cached {
        return Ok(cameras);
    }
    let cameras = fetcher::fetch_camera_list(&state.client).await?;
    *state.camera_cache.lock().await = Some(cameras.clone());
    Ok(cameras)
}

#[tauri::command]
pub async fn refresh_camera_list(state: State<'_, AppState>) -> Result<Vec<CameraInfo>, AppError> {
    let cameras = fetcher::fetch_camera_list(&state.client).await?;
    *state.camera_cache.lock().await = Some(cameras.clone());
    Ok(cameras)
}

#[tauri::command]
pub async fn fetch_image(
    state: State<'_, AppState>,
    camera_id: u32,
    image_url: String,
    previous_hash: Option<u64>,
    save_to_disk: bool,
    save_path: String,
    max_snapshots: usize,
) -> Result<CameraImageFetchOutcome, AppError> {
    let camera = CameraInfo {
        id: camera_id,
        name: String::new(),
        district: String::new(),
        locality: String::new(),
        image_url,
    };
    fetcher::fetch_camera_image(
        &state.client,
        &camera,
        save_to_disk,
        save_path.into(),
        max_snapshots,
        previous_hash,
    )
    .await
    .map_err(AppError::from)
}
