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

pub fn show(ui: &mut egui::Ui, state: &mut AppState) {
    ui.heading("Settings");
    ui.separator();

    egui::ScrollArea::vertical().show(ui, |ui| {
        // ── Districts ─────────────────────────────────────────────────────────
        egui::CollapsingHeader::new("Districts")
            .default_open(true)
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
                    let mut title_size = state.pending_config.camera_title_font_size as f64;
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
                    let mut r = state.pending_config.camera_title_rgb[0] as f64;
                    let mut g = state.pending_config.camera_title_rgb[1] as f64;
                    let mut b = state.pending_config.camera_title_rgb[2] as f64;

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
                let mut r = state.pending_config.app_background_rgb[0] as f64;
                let mut g = state.pending_config.app_background_rgb[1] as f64;
                let mut b = state.pending_config.app_background_rgb[2] as f64;

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

        ui.add_space(8.0);
        ui.separator();

        // ── Apply / Cancel ────────────────────────────────────────────────────
        ui.horizontal(|ui| {
            if ui.button("Apply").clicked() {
                state.apply_settings();
            }
            if ui.button("Cancel").clicked() {
                state.pending_config = state.config.clone();
                state.settings_open = false;
            }
        });

        // Extra tail padding so the final controls can scroll slightly above
        // the panel edge.
        ui.add_space(18.0);
    });
}
