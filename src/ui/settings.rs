use crate::app::AppState;
use std::path::PathBuf;

fn color_swatch(ui: &mut egui::Ui, rgb: [u8; 3]) {
    let (rect, _) = ui.allocate_exact_size(egui::vec2(18.0, 18.0), egui::Sense::hover());
    ui.painter()
        .rect_filled(rect, 3.0, egui::Color32::from_rgb(rgb[0], rgb[1], rgb[2]));
    ui.painter().rect_stroke(
        rect,
        3.0,
        egui::Stroke::new(1.0, egui::Color32::from_gray(90)),
        egui::StrokeKind::Inside,
    );
}

#[allow(
    clippy::cast_precision_loss,     // usize/u64→f64 for slider ranges; values are always small
    clippy::cast_possible_truncation, // f64→usize/u64/u8 slider round-trip; values stay in range
    clippy::cast_sign_loss,          // f64→u8/usize slider round-trip; slider min is 0
    clippy::too_many_lines
)]
pub fn show(ui: &mut egui::Ui, state: &mut AppState) {
    ui.heading("Settings");
    ui.separator();

    // ── Pinned footer (Apply / Cancel / version) ──────────────────────────────
    // Must be declared before the ScrollArea so egui carves the space first.
    egui::Panel::bottom("settings_footer")
        .frame(
            egui::Frame::new()
                .fill(ui.visuals().panel_fill)
                .inner_margin(egui::Margin::symmetric(8, 6)),
        )
        .show_inside(ui, |ui| {
            ui.separator();
            ui.add_space(2.0);
            ui.horizontal(|ui| {
                if ui.button("Apply").clicked() {
                    state.apply_settings();
                }
                if ui.button("Cancel").clicked() {
                    state.pending_config = state.config.clone();
                    state.settings_open = false;
                }
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(
                        egui::RichText::new(concat!(
                            "v",
                            env!("CARGO_PKG_VERSION"),
                            " · built ",
                            env!("BUILD_DATE"),
                        ))
                        .small()
                        .color(egui::Color32::from_gray(100)),
                    );
                });
            });
        });

    egui::ScrollArea::vertical().show(ui, |ui| {
        // ── Districts ─────────────────────────────────────────────────────────
        egui::CollapsingHeader::new("Districts")
            .default_open(false)
            .show(ui, |ui| {
                if state.all_districts.is_empty() {
                    ui.label("Camera list not loaded yet.");
                } else {
                    // Clone to avoid holding a borrow on `state` during iteration.
                    let districts = state.all_districts.clone();
                    for district in &districts {
                        let mut checked =
                            state.pending_config.selected_districts.contains(district);
                        if ui.checkbox(&mut checked, district).changed() {
                            if checked {
                                state
                                    .pending_config
                                    .selected_districts
                                    .insert(district.clone());
                            } else {
                                state
                                    .pending_config
                                    .selected_districts
                                    .remove(district.as_str());
                            }
                        }
                    }
                }
            });

        ui.separator();

        // ── Cameras ───────────────────────────────────────────────────────────
        egui::CollapsingHeader::new("Cameras")
            .default_open(false)
            .show(ui, |ui| {
                if state.all_cameras.is_empty() {
                    ui.label("Camera list not loaded yet.");
                } else {
                    ui.label(
                        egui::RichText::new(
                            "🔼/🔽 to reorder · uncheck to hide · takes effect on Apply",
                        )
                        .small()
                        .color(egui::Color32::from_gray(140)),
                    );
                    ui.add_space(4.0);

                    // Clone everything we need to avoid holding borrows on `state`.
                    let all_cameras = state.all_cameras.clone();
                    let selected_districts = state.pending_config.selected_districts.clone();
                    let camera_order = state.pending_config.camera_order.clone();
                    let multi_district = selected_districts.len() > 1;

                    // Build the ordered list of camera IDs for active districts
                    // (hidden cameras are included so they keep their position).
                    let active_ids: Vec<u32> = {
                        // Cameras that belong to an active district.
                        let active_cams: Vec<_> = all_cameras
                            .iter()
                            .filter(|c| selected_districts.contains(&c.district))
                            .collect();

                        if camera_order.is_empty() {
                            // Default: district alphabetically, then camera name.
                            let mut sorted = active_cams.clone();
                            sorted.sort_by(|a, b| {
                                a.district.cmp(&b.district).then(a.name.cmp(&b.name))
                            });
                            sorted.iter().map(|c| c.id).collect()
                        } else {
                            // Respect stored order; append new cameras at the end.
                            let mut ids: Vec<u32> = camera_order
                                .iter()
                                .filter(|&&id| active_cams.iter().any(|c| c.id == id))
                                .copied()
                                .collect();
                            for cam in &active_cams {
                                if !camera_order.contains(&cam.id) {
                                    ids.push(cam.id);
                                }
                            }
                            ids
                        }
                    };

                    let n = active_ids.len();
                    for (pos, &cam_id) in active_ids.clone().iter().enumerate() {
                        let Some(cam) = all_cameras.iter().find(|c| c.id == cam_id) else {
                            continue;
                        };

                        ui.horizontal(|ui| {
                            // ▲ / ▼ reorder buttons.
                            if ui
                                .add_enabled(pos > 0, egui::Button::new("🔼").small())
                                .clicked()
                            {
                                let order = &mut state.pending_config.camera_order;
                                if order.len() != n {
                                    order.clone_from(&active_ids);
                                }
                                if let Some(i) = order.iter().position(|&id| id == cam_id)
                                    && i > 0
                                {
                                    order.swap(i, i - 1);
                                }
                            }
                            if ui
                                .add_enabled(pos < n - 1, egui::Button::new("🔽").small())
                                .clicked()
                            {
                                let order = &mut state.pending_config.camera_order;
                                if order.len() != n {
                                    order.clone_from(&active_ids);
                                }
                                if let Some(i) = order.iter().position(|&id| id == cam_id)
                                    && i + 1 < order.len()
                                {
                                    order.swap(i, i + 1);
                                }
                            }

                            // Visibility checkbox.
                            let mut visible =
                                !state.pending_config.hidden_camera_ids.contains(&cam_id);
                            let label = if multi_district {
                                format!("{} — {}", cam.name, cam.district)
                            } else {
                                cam.name.clone()
                            };
                            if ui.checkbox(&mut visible, label).changed() {
                                if visible {
                                    state.pending_config.hidden_camera_ids.remove(&cam_id);
                                } else {
                                    state.pending_config.hidden_camera_ids.insert(cam_id);
                                }
                            }
                        });
                    }
                }
            });

        ui.separator();

        // ── Display ───────────────────────────────────────────────────────────
        egui::CollapsingHeader::new("Display")
            .default_open(true)
            .show(ui, |ui| {
                ui.label("Refresh interval (seconds):");
                let mut interval = state.pending_config.refresh_interval_secs as f64;
                if ui
                    .add(egui::Slider::new(&mut interval, 10.0..=300.0).integer())
                    .changed()
                {
                    state.pending_config.refresh_interval_secs = interval as u64;
                }

                ui.add_space(4.0);
                ui.label("Columns:");
                let mut cols = state.pending_config.column_count as f64;
                if ui
                    .add(egui::Slider::new(&mut cols, 1.0..=6.0).integer())
                    .changed()
                {
                    state.pending_config.column_count = cols as usize;
                }

                ui.add_space(4.0);
                ui.label("Grid spacing:");
                ui.add(
                    egui::Slider::new(&mut state.pending_config.grid_spacing, 0.0..=16.0)
                        .suffix(" px"),
                );

                ui.add_space(4.0);
                ui.checkbox(
                    &mut state.pending_config.show_camera_titles,
                    "Show camera titles",
                );

                ui.add_space(4.0);
                ui.label("Camera title font size:");
                ui.add_enabled_ui(state.pending_config.show_camera_titles, |ui| {
                    let mut title_size = f64::from(state.pending_config.camera_title_font_size);
                    if ui
                        .add(
                            egui::Slider::new(&mut title_size, 9.0..=24.0)
                                .integer()
                                .suffix(" pt"),
                        )
                        .changed()
                    {
                        state.pending_config.camera_title_font_size = title_size as f32;
                    }
                });

                ui.add_space(4.0);
                ui.label("Camera title color:");
                ui.add_enabled_ui(state.pending_config.show_camera_titles, |ui| {
                    let mut r = f64::from(state.pending_config.camera_title_rgb[0]);
                    let mut g = f64::from(state.pending_config.camera_title_rgb[1]);
                    let mut b = f64::from(state.pending_config.camera_title_rgb[2]);

                    let mut changed = false;
                    changed |= ui
                        .add(egui::Slider::new(&mut r, 0.0..=255.0).integer().text("R"))
                        .changed();
                    changed |= ui
                        .add(egui::Slider::new(&mut g, 0.0..=255.0).integer().text("G"))
                        .changed();
                    changed |= ui
                        .add(egui::Slider::new(&mut b, 0.0..=255.0).integer().text("B"))
                        .changed();

                    if changed {
                        state.pending_config.camera_title_rgb = [r as u8, g as u8, b as u8];
                    }

                    ui.horizontal(|ui| {
                        ui.label("Preview:");
                        color_swatch(ui, state.pending_config.camera_title_rgb);
                    });
                });

                ui.add_space(4.0);
                ui.label("Camera image aspect ratio:");
                ui.horizontal(|ui| {
                    let is_5_4 = state.pending_config.camera_aspect_ratio == [320, 256];
                    let is_4_3 = state.pending_config.camera_aspect_ratio == [4, 3];
                    let is_16_9 = state.pending_config.camera_aspect_ratio == [16, 9];

                    if ui.selectable_label(is_5_4, "5:4").clicked() {
                        state.pending_config.camera_aspect_ratio = [320, 256];
                    }
                    if ui.selectable_label(is_4_3, "4:3").clicked() {
                        state.pending_config.camera_aspect_ratio = [4, 3];
                    }
                    if ui.selectable_label(is_16_9, "16:9").clicked() {
                        state.pending_config.camera_aspect_ratio = [16, 9];
                    }
                });

                ui.add_space(4.0);
                ui.label("App background color (grid only):");
                let mut r = f64::from(state.pending_config.app_background_rgb[0]);
                let mut g = f64::from(state.pending_config.app_background_rgb[1]);
                let mut b = f64::from(state.pending_config.app_background_rgb[2]);

                let mut changed = false;
                changed |= ui
                    .add(egui::Slider::new(&mut r, 0.0..=255.0).integer().text("R"))
                    .changed();
                changed |= ui
                    .add(egui::Slider::new(&mut g, 0.0..=255.0).integer().text("G"))
                    .changed();
                changed |= ui
                    .add(egui::Slider::new(&mut b, 0.0..=255.0).integer().text("B"))
                    .changed();

                if changed {
                    state.pending_config.app_background_rgb = [r as u8, g as u8, b as u8];
                }

                ui.horizontal(|ui| {
                    ui.label("Preview:");
                    color_swatch(ui, state.pending_config.app_background_rgb);
                });
            });

        ui.separator();

        // ── Disk Save ─────────────────────────────────────────────────────────
        egui::CollapsingHeader::new("Disk Save")
            .default_open(true)
            .show(ui, |ui| {
                ui.checkbox(
                    &mut state.pending_config.save_to_disk,
                    "Save images to disk",
                );

                if state.pending_config.save_to_disk {
                    ui.add_space(4.0);
                    ui.label("Save path:");
                    ui.horizontal(|ui| {
                        let browse_button_width = 88.0;
                        let input_width = (ui.available_width() - browse_button_width).max(120.0);
                        ui.add_sized(
                            [input_width, 0.0],
                            egui::TextEdit::singleline(&mut state.pending_config.save_path),
                        );
                        if ui.button("Browse…").clicked() {
                            let start = PathBuf::from(&state.pending_config.save_path);
                            let mut dialog = rfd::FileDialog::new();
                            if start.is_dir() {
                                dialog = dialog.set_directory(start);
                            }
                            if let Some(path) = dialog.pick_folder() {
                                state.pending_config.save_path = path.to_string_lossy().to_string();
                            }
                        }
                    });

                    ui.add_space(4.0);
                    ui.label("Max snapshots per camera:");
                    let mut max = state.pending_config.max_snapshots as f64;
                    if ui
                        .add(egui::Slider::new(&mut max, 1.0..=20.0).integer())
                        .changed()
                    {
                        state.pending_config.max_snapshots = max as usize;
                    }

                    ui.add_space(6.0);
                    // "Clear Cache" — enabled only when the save directory actually exists.
                    let cache_path = PathBuf::from(&state.pending_config.save_path);
                    let cache_exists = cache_path.is_dir();
                    ui.add_enabled_ui(cache_exists, |ui| {
                        if ui
                            .button("🗑 Clear Cache")
                            .on_hover_text("Delete all saved images in the save directory")
                            .clicked()
                            && let Err(e) = std::fs::remove_dir_all(&cache_path)
                        {
                            log::warn!("Failed to clear cache: {e}");
                        }
                    });
                    if !cache_exists {
                        ui.label(
                            egui::RichText::new("Cache directory does not exist yet.")
                                .small()
                                .color(egui::Color32::from_gray(140)),
                        );
                    }
                }
            });
    });
}
