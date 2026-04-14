use crate::camera::CameraInfo;
use reqwest::Client;
use serde::Deserialize;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::path::{Path, PathBuf};
use thiserror::Error;

// ── Error type ───────────────────────────────────────────────────────────────

#[derive(Debug, Error)]
pub enum FetchError {
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),
    #[error("Image decode error: {0}")]
    Decode(String),
    #[error("JSON parse error: {0}")]
    Json(String),
}

// ── GeoJSON wire types ───────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct GeoJsonRoot {
    pub features: Vec<GeoFeature>,
}

#[derive(Debug, Deserialize)]
pub struct GeoFeature {
    pub properties: CameraProperties,
}

#[derive(Debug, Deserialize)]
pub struct CameraProperties {
    pub id: u32,
    pub description: String,
    pub district: String,
    pub locality: String,
    pub image_url: String,
    // All other GeoJSON fields (e.g. postcode) are ignored by serde automatically.
}

// ── HTTP client ──────────────────────────────────────────────────────────────

/// Thin wrapper around `reqwest::Client`.  Clone is cheap (Arc internally).
#[derive(Clone)]
pub struct HttpClient(Client);

impl HttpClient {
    pub fn new() -> Self {
        Self(
            Client::builder()
                .timeout(std::time::Duration::from_secs(15))
                .build()
                .expect("Failed to build HTTP client"),
        )
    }
}

// ── Network functions ────────────────────────────────────────────────────────

const GEOJSON_URL: &str = "https://data.qldtraffic.qld.gov.au/webcameras.geojson";

/// Fetch the QLD traffic camera list from the live `GeoJSON` feed.
pub async fn fetch_camera_list(client: &HttpClient) -> Result<Vec<CameraInfo>, FetchError> {
    let body = client.0.get(GEOJSON_URL).send().await?.text().await?;
    let root: GeoJsonRoot =
        serde_json::from_str(&body).map_err(|e| FetchError::Json(e.to_string()))?;
    Ok(root
        .features
        .into_iter()
        .map(|f| CameraInfo {
            id: f.properties.id,
            name: f.properties.description,
            district: f.properties.district,
            locality: f.properties.locality,
            image_url: f.properties.image_url,
        })
        .collect())
}

#[derive(Clone)]
pub enum CameraImageFetchOutcome {
    Changed { hash: u64, image: egui::ColorImage },
    Unchanged { hash: u64 },
}

/// Fetch a single camera image, optionally save to disk, and decode to an
/// `egui::ColorImage` ready for GPU upload on the UI thread.
pub async fn fetch_camera_image(
    client: &HttpClient,
    camera: &CameraInfo,
    save_to_disk: bool,
    save_path: PathBuf,
    max_snapshots: usize,
    previous_hash: Option<u64>,
) -> Result<CameraImageFetchOutcome, FetchError> {
    // Append unix-millisecond timestamp to bust CDN caches.
    let ts = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_millis())
        .unwrap_or(0);
    let url = format!("{}?{}", camera.image_url, ts);

    let bytes = client.0.get(&url).send().await?.bytes().await?;

    let mut hasher = DefaultHasher::new();
    bytes.hash(&mut hasher);
    let hash = hasher.finish();

    if previous_hash == Some(hash) {
        return Ok(CameraImageFetchOutcome::Unchanged { hash });
    }

    // Optional disk save — fire and forget, errors are logged not propagated.
    if save_to_disk && !save_path.as_os_str().is_empty() {
        let stem = url_stem(&camera.image_url).to_string();
        let bytes_for_save = bytes.clone();
        drop(tokio::task::spawn_blocking(move || {
            save_image_rolling(&bytes_for_save, &stem, &save_path, max_snapshots);
        }));
    }

    // Decode JPEG → RGBA8 → egui::ColorImage (on the tokio worker thread).
    let dyn_img = image::load_from_memory_with_format(&bytes, image::ImageFormat::Jpeg)
        .map_err(|e| FetchError::Decode(e.to_string()))?;
    let rgba = dyn_img.to_rgba8();
    let (w, h) = rgba.dimensions();
    Ok(CameraImageFetchOutcome::Changed {
        hash,
        image: egui::ColorImage::from_rgba_unmultiplied([w as usize, h as usize], rgba.as_raw()),
    })
}

// ── Disk save helpers ────────────────────────────────────────────────────────

/// Extract the filename stem from a camera image URL.
/// e.g. `https://…/Wide_Bay/bargara-davidson-east.jpg?ts` → `bargara-davidson-east`
fn url_stem(image_url: &str) -> &str {
    let path = image_url.split('?').next().unwrap_or(image_url);
    let filename = path.split('/').next_back().unwrap_or("camera");
    filename.strip_suffix(".jpg").unwrap_or(filename)
}

/// Write `bytes` to `{save_path}/{stem}_{unix_secs}.jpg`, then delete any
/// files for this stem beyond `max_snapshots` (oldest first).
fn save_image_rolling(bytes: &[u8], stem: &str, save_path: &Path, max_snapshots: usize) {
    if let Err(e) = try_save_image_rolling(bytes, stem, save_path, max_snapshots) {
        log::warn!("Disk save failed for '{stem}': {e}");
    }
}

fn try_save_image_rolling(
    bytes: &[u8],
    stem: &str,
    save_path: &Path,
    max_snapshots: usize,
) -> std::io::Result<()> {
    std::fs::create_dir_all(save_path)?;

    let ts = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    std::fs::write(save_path.join(format!("{stem}_{ts}.jpg")), bytes)?;

    // Collect all files for this camera stem.
    let prefix = format!("{stem}_");
    let mut matches: Vec<_> = std::fs::read_dir(save_path)?
        .filter_map(Result::ok)
        .filter(|e| {
            let name = e.file_name();
            let name = name.to_string_lossy();
            name.starts_with(&prefix) && name.ends_with(".jpg")
        })
        .collect();

    // Delete oldest files beyond the rolling limit.
    if matches.len() > max_snapshots {
        matches.sort_by_key(|e| e.metadata().and_then(|m| m.modified()).ok());
        let to_delete = matches.len() - max_snapshots;
        for entry in matches.iter().take(to_delete) {
            if let Err(e) = std::fs::remove_file(entry.path()) {
                log::warn!(
                    "Failed to delete old snapshot {}: {e}",
                    entry.path().display()
                );
            }
        }
    }

    Ok(())
}

// ── Tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    const FIXTURE: &str = r#"{
        "type": "FeatureCollection",
        "features": [
            {
                "type": "Feature",
                "geometry": {"type": "Point", "coordinates": [152.36, -24.87]},
                "properties": {
                    "id": 42,
                    "description": "Test Camera North",
                    "district": "Wide Bay/Burnett",
                    "locality": "Bundaberg",
                    "postcode": "4670",
                    "direction": "North",
                    "image_url": "https://cameras.qldtraffic.qld.gov.au/Wide_Bay/test-camera.jpg",
                    "extra_unknown_field": "should be ignored"
                }
            }
        ]
    }"#;

    #[test]
    fn geojson_parses_correctly() {
        let root: GeoJsonRoot = serde_json::from_str(FIXTURE).expect("parse failed");
        assert_eq!(root.features.len(), 1);
        let p = &root.features[0].properties;
        assert_eq!(p.id, 42);
        assert_eq!(p.description, "Test Camera North");
        assert_eq!(p.district, "Wide Bay/Burnett");
        assert_eq!(p.locality, "Bundaberg");
        assert_eq!(
            p.image_url,
            "https://cameras.qldtraffic.qld.gov.au/Wide_Bay/test-camera.jpg"
        );
    }

    #[test]
    fn unknown_fields_are_ignored() {
        // "extra_unknown_field" and "direction" are not in our struct — must not panic.
        let result: Result<GeoJsonRoot, _> = serde_json::from_str(FIXTURE);
        assert!(result.is_ok(), "Unknown fields should be silently ignored");
    }

    #[test]
    fn url_stem_extracts_correctly() {
        assert_eq!(
            url_stem("https://cameras.qldtraffic.qld.gov.au/Wide_Bay/bargara-davidson-east.jpg"),
            "bargara-davidson-east"
        );
        assert_eq!(
            url_stem(
                "https://cameras.qldtraffic.qld.gov.au/Wide_Bay/bargara-davidson-east.jpg?1740304600643"
            ),
            "bargara-davidson-east"
        );
    }
}
