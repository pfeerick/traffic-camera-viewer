use egui::TextureHandle;
use poll_promise::Promise;

/// Describes a single traffic camera from the GeoJSON feed.
#[derive(Debug, Clone)]
pub struct CameraInfo {
    pub id: u32,
    pub name: String,     // `description` field in GeoJSON
    pub district: String,
    pub locality: String,
    pub image_url: String, // base URL; cache-buster appended at fetch time
}

/// Per-camera runtime state held in AppState.
pub struct CameraState {
    pub info: CameraInfo,
    pub image_state: ImageState,
}

/// State machine for a single camera's image load cycle.
pub enum ImageState {
    /// No fetch started yet (initial state, or after filter change).
    Idle,
    /// A fetch is in-flight.
    Loading(Promise<Result<egui::ColorImage, crate::fetcher::FetchError>>),
    /// Image decoded and uploaded to the GPU.
    Ready(TextureHandle),
    /// Last fetch failed; message is displayed in the grid cell.
    Error(String),
}
