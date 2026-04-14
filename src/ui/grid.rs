use crate::app::AppState;
use crate::camera::ImageState;

pub fn show(ui: &mut egui::Ui, state: &mut AppState) {
    // Guard against tiny bottom-edge bleed when scrolled content meets the
    // status bar panel boundary.
    let mut clip_rect = ui.clip_rect();
    clip_rect.max.y = (clip_rect.max.y - 2.0).max(clip_rect.min.y);
    ui.set_clip_rect(clip_rect);

    // Paint the grid area background only (status bar uses its own panel frame).
    let [r, g, b] = state.config.app_background_rgb;
    ui.painter()
        .rect_filled(ui.max_rect(), 0.0, egui::Color32::from_rgb(r, g, b));

    // ── Empty-state messages ──────────────────────────────────────────────────
    if state.cameras.is_empty() {
        ui.centered_and_justified(|ui| {
            if let Some(err) = &state.geojson_error {
                ui.label(format!("⚠ Failed to load camera list:\n{err}"));
            } else if state.geojson_promise.is_some() {
                ui.vertical_centered(|ui| {
                    ui.spinner();
                    ui.label("Loading camera list…");
                });
            } else {
                ui.label("No cameras selected. Open Settings to choose a district.");
            }
        });
        return;
    }

    // ── Scrollable camera grid ────────────────────────────────────────────────
    // Cell dimensions are computed INSIDE the ScrollArea so that
    // ui.available_width() already excludes the scrollbar.
    let cols = state.config.column_count.max(1);
    let [aspect_w, aspect_h] = state.config.camera_aspect_ratio;
    let aspect_h_over_w = (aspect_h as f32 / aspect_w as f32).max(0.1);
    let [title_r, title_g, title_b] = state.config.camera_title_rgb;
    let title_color = egui::Color32::from_rgb(title_r, title_g, title_b);

    egui::ScrollArea::vertical()
        .auto_shrink([false, false])
        .show(ui, |ui| {
            let h_spacing = state.config.grid_spacing;
            let v_spacing = state.config.grid_spacing;
            // We apply spacing manually so the slider is the only inter-tile gap source.
            ui.spacing_mut().item_spacing = egui::vec2(0.0, 0.0);
            let available_w = ui.available_width();
            let cell_w = ((available_w - h_spacing * (cols as f32 - 1.0)) / cols as f32).max(80.0);
            let cell_h = cell_w * aspect_h_over_w;
            let cell_size = egui::vec2(cell_w, cell_h);

            let num_cameras = state.cameras.len();
            let mut row_start = 0;

            while row_start < num_cameras {
                let row_end = (row_start + cols).min(num_cameras);

                ui.horizontal(|ui| {
                    for idx in row_start..row_end {
                        let camera_name = state.cameras[idx].info.name.clone();
                        let camera_locality = state.cameras[idx].info.locality.clone();

                        ui.vertical(|ui| {
                            ui.set_width(cell_w);
                            ui.spacing_mut().item_spacing.y = 2.0;

                            // Optional title block above each image.
                            if state.config.show_camera_titles {
                                let title_size = state.config.camera_title_font_size;
                                ui.label(
                                    egui::RichText::new(&camera_locality)
                                        .strong()
                                        .size(title_size)
                                        .color(title_color),
                                )
                                .on_hover_text(&camera_name);
                            }

                            // Image area
                            match &state.cameras[idx].image_state {
                                ImageState::Idle => {
                                    let (rect, response) =
                                        ui.allocate_exact_size(cell_size, egui::Sense::hover());
                                    ui.painter().rect_filled(
                                        rect,
                                        4.0,
                                        egui::Color32::from_gray(40),
                                    );
                                    response.on_hover_text(&camera_name);
                                }

                                ImageState::Loading(_) => {
                                    let (rect, response) =
                                        ui.allocate_exact_size(cell_size, egui::Sense::hover());
                                    ui.painter().rect_filled(
                                        rect,
                                        4.0,
                                        egui::Color32::from_gray(40),
                                    );
                                    // Overlay spinner in the centre of the placeholder.
                                    let spinner_rect = egui::Rect::from_center_size(
                                        rect.center(),
                                        egui::vec2(32.0, 32.0),
                                    );
                                    ui.put(spinner_rect, egui::Spinner::new());
                                    response.on_hover_text(&camera_name);
                                }

                                ImageState::Ready(handle) => {
                                    let sized = egui::load::SizedTexture::from_handle(handle);
                                    ui.add(
                                        egui::Image::new(sized)
                                            .fit_to_exact_size(cell_size)
                                            .maintain_aspect_ratio(true),
                                    )
                                    .on_hover_text(&camera_name);
                                }

                                ImageState::Error(msg) => {
                                    let (rect, response) =
                                        ui.allocate_exact_size(cell_size, egui::Sense::hover());
                                    ui.painter().rect_filled(
                                        rect,
                                        4.0,
                                        egui::Color32::from_rgb(120, 20, 20),
                                    );
                                    ui.painter().text(
                                        rect.center(),
                                        egui::Align2::CENTER_CENTER,
                                        msg.as_str(),
                                        egui::FontId::proportional(11.0),
                                        egui::Color32::WHITE,
                                    );
                                    response.on_hover_text(&camera_name);
                                }
                            }
                        });

                        // Add horizontal gap between columns (except after the last one).
                        if idx < row_end - 1 {
                            ui.add_space(h_spacing);
                        }
                    }
                });

                if row_end < num_cameras {
                    ui.add_space(v_spacing);
                }
                row_start = row_end;
            }
        });
}
