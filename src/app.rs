use chrono::{DateTime, Duration, Local};
use poll_promise::Promise;
use std::path::PathBuf;

use crate::camera::{CameraInfo, CameraState, ImageState};
use crate::config::{AppConfig, load_config, save_config};
use crate::fetcher::{
    CameraImageFetchOutcome, FetchError, HttpClient, fetch_camera_image, fetch_camera_list,
};

pub struct AppState {
    pub config: AppConfig,
    /// Pending edits in the settings panel — applied only when user clicks "Apply".
    pub pending_config: AppConfig,

    /// Cameras matching the current district filter (what the grid renders).
    pub cameras: Vec<CameraState>,
    /// Full unfiltered camera list, refreshed once on startup.
    pub all_cameras: Vec<CameraInfo>,
    /// Sorted unique district names for the settings checkbox list.
    pub all_districts: Vec<String>,

    pub client: HttpClient,

    /// In-flight `GeoJSON` fetch on startup.
    pub geojson_promise: Option<Promise<Result<Vec<CameraInfo>, FetchError>>>,
    /// Non-None when the `GeoJSON` fetch failed.
    pub geojson_error: Option<String>,

    pub last_refresh: Option<DateTime<Local>>,
    pub next_refresh: Option<DateTime<Local>>,

    pub settings_open: bool,
}

impl AppState {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let config = load_config();
        let client = HttpClient::new();

        // Kick off the GeoJSON fetch immediately; the promise will be polled
        // in the render loop.
        let ctx = cc.egui_ctx.clone();
        let client2 = client.clone();
        let geojson_promise = Some(Promise::spawn_async(async move {
            let result = fetch_camera_list(&client2).await;
            ctx.request_repaint(); // wake the UI when data arrives
            result
        }));

        Self {
            pending_config: config.clone(),
            config,
            cameras: Vec::new(),
            all_cameras: Vec::new(),
            all_districts: Vec::new(),
            client,
            geojson_promise,
            geojson_error: None,
            last_refresh: None,
            next_refresh: None,
            settings_open: false,
        }
    }

    // ── Refresh timer ──────────────────────────────────────────────────────

    pub fn tick_refresh_timer(&mut self, ctx: &egui::Context) {
        if let Some(next) = self.next_refresh {
            let now = Local::now();
            if now >= next {
                self.trigger_refresh_all(ctx);
            } else {
                // Keep waking the UI once per second so the countdown updates.
                let remaining = (next - now).to_std().unwrap_or_default();
                ctx.request_repaint_after(remaining.min(std::time::Duration::from_secs(1)));
            }
        }
    }

    /// Spawn image fetch promises for every camera in the current filter and
    /// reset the refresh timer.
    #[allow(clippy::cast_possible_wrap)] // refresh_interval_secs is always small (10–300)
    pub fn trigger_refresh_all(&mut self, ctx: &egui::Context) {
        let now = Local::now();
        self.last_refresh = Some(now);
        self.next_refresh = Some(now + Duration::seconds(self.config.refresh_interval_secs as i64));

        let save_to_disk = self.config.save_to_disk;
        let max_snapshots = self.config.max_snapshots;
        let save_path = PathBuf::from(&self.config.save_path);

        for cam in &mut self.cameras {
            let cam_info = cam.info.clone();
            let client = self.client.clone();
            let ctx2 = ctx.clone();
            let save_path2 = save_path.clone();
            let previous_hash = cam.last_image_hash;
            let previous_texture = if let ImageState::Ready(handle) = &cam.image_state {
                Some(handle.clone())
            } else {
                None
            };

            cam.image_state = ImageState::Loading {
                promise: Promise::spawn_async(async move {
                    let result = fetch_camera_image(
                        &client,
                        &cam_info,
                        save_to_disk,
                        save_path2,
                        max_snapshots,
                        previous_hash,
                    )
                    .await;
                    ctx2.request_repaint();
                    result
                }),
                previous: previous_texture,
            };
        }
    }

    // ── Promise polling ────────────────────────────────────────────────────

    /// Poll the startup `GeoJSON` promise; on success builds the camera list and
    /// triggers the first image refresh immediately.
    pub fn poll_geojson(&mut self, ctx: &egui::Context) {
        let is_ready = self
            .geojson_promise
            .as_ref()
            .is_some_and(|p| p.ready().is_some());

        if is_ready {
            // ready() returns &T — clone what we need before dropping the promise.
            let outcome = self.geojson_promise.as_ref().and_then(|p| {
                p.ready().map(|r| match r {
                    Ok(cams) => Ok(cams.clone()),
                    Err(e) => Err(e.to_string()),
                })
            });
            self.geojson_promise = None; // drop the promise

            match outcome {
                Some(Ok(all_cams)) => {
                    let mut districts: Vec<String> = all_cams
                        .iter()
                        .map(|c| c.district.clone())
                        .collect::<std::collections::BTreeSet<_>>()
                        .into_iter()
                        .collect();
                    districts.sort();

                    self.all_cameras = all_cams;
                    self.all_districts = districts;
                    self.rebuild_camera_list();
                    self.next_refresh = Some(Local::now());
                    ctx.request_repaint();
                }
                Some(Err(msg)) => {
                    log::error!("GeoJSON fetch failed: {msg}");
                    self.geojson_error = Some(msg);
                }
                None => {}
            }
        }
    }

    /// Check all per-camera promises; upload decoded images to GPU textures on
    /// the UI thread when they arrive.
    ///
    /// `Promise::ready()` returns `&T`, so we clone the `ColorImage` here before
    /// transitioning the image state (avoiding a borrow-across-replace issue).
    pub fn poll_image_promises(&mut self, ctx: &egui::Context) {
        for cam in &mut self.cameras {
            // Extract the result (cloning what we need) while holding an
            // immutable borrow of image_state, then release that borrow before
            // mutating image_state below.
            let (result, previous_texture) =
                if let ImageState::Loading { promise, previous } = &cam.image_state {
                    (
                        promise.ready().map(|r| match r {
                            Ok(outcome) => Ok(outcome.clone()),
                            Err(e) => Err(e.to_string()),
                        }),
                        previous.clone(),
                    )
                } else {
                    (None, None)
                };

            if let Some(result) = result {
                cam.image_state = match result {
                    Ok(CameraImageFetchOutcome::Changed { hash, image }) => {
                        cam.last_image_hash = Some(hash);
                        // Texture upload MUST happen on the UI thread — this is it.
                        let handle = ctx.load_texture(
                            format!("cam_{}", cam.info.id),
                            image,
                            egui::TextureOptions::LINEAR,
                        );
                        ImageState::Ready(handle)
                    }
                    Ok(CameraImageFetchOutcome::Unchanged { hash }) => {
                        cam.last_image_hash = Some(hash);
                        previous_texture.map_or(ImageState::Idle, ImageState::Ready)
                    }
                    Err(e) => ImageState::Error(e),
                };
            }
        }
    }

    /// Spawn an image fetch promise for a single camera by ID, without
    /// disturbing the global refresh timer or other cameras.
    pub fn trigger_refresh_single(&mut self, ctx: &egui::Context, camera_id: u32) {
        let save_to_disk = self.config.save_to_disk;
        let max_snapshots = self.config.max_snapshots;
        let save_path = PathBuf::from(&self.config.save_path);

        if let Some(cam) = self.cameras.iter_mut().find(|c| c.info.id == camera_id) {
            let cam_info = cam.info.clone();
            let client = self.client.clone();
            let ctx2 = ctx.clone();
            let previous_hash = cam.last_image_hash;
            let previous_texture = if let ImageState::Ready(handle) = &cam.image_state {
                Some(handle.clone())
            } else {
                None
            };

            cam.image_state = ImageState::Loading {
                promise: Promise::spawn_async(async move {
                    let result = fetch_camera_image(
                        &client,
                        &cam_info,
                        save_to_disk,
                        save_path,
                        max_snapshots,
                        previous_hash,
                    )
                    .await;
                    ctx2.request_repaint();
                    result
                }),
                previous: previous_texture,
            };
        }
    }

    // ── Camera list management ─────────────────────────────────────────────

    /// Rebuild `cameras` from `all_cameras` applying the current district filter,
    /// hidden-camera filter, and custom display order. All camera image states are
    /// reset to `Idle`.
    pub fn rebuild_camera_list(&mut self) {
        let mut cams: Vec<CameraState> = self
            .all_cameras
            .iter()
            .filter(|c| {
                self.config.selected_districts.contains(&c.district)
                    && !self.config.hidden_camera_ids.contains(&c.id)
            })
            .map(|c| CameraState {
                info: c.clone(),
                image_state: ImageState::Idle,
                last_image_hash: None,
            })
            .collect();

        if !self.config.camera_order.is_empty() {
            let order_map: std::collections::HashMap<u32, usize> = self
                .config
                .camera_order
                .iter()
                .enumerate()
                .map(|(i, &id)| (id, i))
                .collect();
            cams.sort_by_key(|c| order_map.get(&c.info.id).copied().unwrap_or(usize::MAX));
        }

        self.cameras = cams;
    }

    /// Copy pending settings → active config, persist to disk, rebuild cameras,
    /// and schedule an immediate refresh.
    pub fn apply_settings(&mut self) {
        self.config = self.pending_config.clone();
        save_config(&self.config);
        self.rebuild_camera_list();
        self.next_refresh = Some(Local::now());
    }
}

// ── eframe::App ───────────────────────────────────────────────────────────────

impl eframe::App for AppState {
    /// eframe 0.34 calls `ui` (not `update`) — the provided `ui` is the viewport.
    /// Panels carved out via `show_inside` narrow the remaining area; the grid
    /// fills whatever is left.
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let ctx = ui.ctx().clone();

        // 1. Drive the auto-refresh countdown.
        self.tick_refresh_timer(&ctx);

        // 2. Check the GeoJSON startup promise.
        self.poll_geojson(&ctx);

        // 3. Check per-camera image promises.
        self.poll_image_promises(&ctx);

        // 4. Render panels — order matters.
        // Bottom is carved first so the status bar spans full width,
        // including below the optional right settings panel.
        egui::Panel::bottom("status_bar")
            .frame(
                egui::Frame::new()
                    .fill(ui.visuals().panel_fill)
                    .inner_margin(egui::Margin::symmetric(8, 6)),
            )
            .show_inside(ui, |ui| crate::ui::statusbar::show(ui, self));

        if self.settings_open {
            egui::Panel::right("settings_panel")
                .default_size(340.0)
                .min_size(300.0)
                .max_size(360.0)
                .show_inside(ui, |ui| crate::ui::settings::show(ui, self));
        }

        // Remaining area → camera grid.
        let available = ui.available_size();
        let grid_action = ui
            .allocate_ui_with_layout(available, egui::Layout::top_down(egui::Align::Min), |ui| {
                ui.set_clip_rect(ui.max_rect());
                crate::ui::grid::show(ui, self)
            })
            .inner;

        if let Some(action) = grid_action {
            match action {
                crate::ui::grid::GridAction::RefreshCamera(id) => {
                    self.trigger_refresh_single(&ctx, id);
                }
                crate::ui::grid::GridAction::HideCamera(id) => {
                    self.config.hidden_camera_ids.insert(id);
                    // Keep pending_config in sync so the settings panel reflects the change.
                    self.pending_config.hidden_camera_ids.insert(id);
                    save_config(&self.config);
                    // Remove only this camera rather than rebuilding the whole list, so
                    // other cameras keep their loaded textures without disruption.
                    self.cameras.retain(|c| c.info.id != id);
                    ctx.request_repaint();
                }
            }
        }
    }

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        save_config(&self.config);
    }
}
