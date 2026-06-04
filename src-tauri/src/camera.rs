use serde::{Deserialize, Serialize};

/// Describes a single traffic camera from the GeoJSON feed.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CameraInfo {
    pub id: u32,
    pub name: String,
    pub district: String,
    pub locality: String,
    pub image_url: String,
}
