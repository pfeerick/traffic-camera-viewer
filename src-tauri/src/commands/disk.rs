use crate::error::AppError;
use serde::Serialize;
use std::path::Path;

#[derive(Debug, Serialize)]
pub struct CacheInfo {
    pub exists: bool,
    pub file_count: usize,
    pub total_bytes: u64,
}

#[tauri::command]
pub async fn clear_cache(path: String) -> Result<(), AppError> {
    let p = Path::new(&path);
    if !p.exists() {
        return Ok(());
    }
    for entry in std::fs::read_dir(p)?.filter_map(Result::ok) {
        let ep = entry.path();
        if ep.is_file() {
            std::fs::remove_file(&ep)?;
        }
    }
    Ok(())
}

#[tauri::command]
pub async fn get_cache_info(path: String) -> Result<CacheInfo, AppError> {
    let p = Path::new(&path);
    if !p.exists() {
        return Ok(CacheInfo {
            exists: false,
            file_count: 0,
            total_bytes: 0,
        });
    }
    let mut file_count = 0usize;
    let mut total_bytes = 0u64;
    for entry in std::fs::read_dir(p)?.filter_map(Result::ok) {
        if entry.path().is_file() {
            file_count += 1;
            total_bytes += entry.metadata().map(|m| m.len()).unwrap_or(0);
        }
    }
    Ok(CacheInfo {
        exists: true,
        file_count,
        total_bytes,
    })
}
