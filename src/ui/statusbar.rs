use crate::app::AppState;
use chrono::Local;

pub fn show(ui: &mut egui::Ui, state: &mut AppState) {
    ui.horizontal(|ui| {
        // ── Primary action (left) ─────────────────────────────────────────────
        let refresh_enabled = !state.cameras.is_empty();
        let refresh_response = ui
            .add_enabled_ui(refresh_enabled, |ui| ui.button("Refresh Now"))
            .inner;
        if refresh_response.clicked() {
            state.trigger_refresh_all(ui.ctx());
            refresh_response.surrender_focus();
        }

        ui.separator();

        // ── Status information (middle) ───────────────────────────────────────
        match state.last_refresh {
            Some(t) => {
                ui.label(format!("Last: {}", t.format("%H:%M:%S")));
            }
            None => {
                ui.label("Last: —");
            }
        }

        ui.separator();

        // ── Next-refresh progress bar ─────────────────────────────────────────
        match (state.last_refresh, state.next_refresh) {
            (Some(last), Some(next)) => {
                let now = Local::now();
                let interval_secs = (next - last)
                    .to_std()
                    .map_or(0.0, |duration| duration.as_secs_f32());
                let remaining_secs_f32 = (next - now)
                    .to_std()
                    .map_or(0.0, |duration| duration.as_secs_f32());
                let progress = if interval_secs > 0.0 {
                    (remaining_secs_f32 / interval_secs).clamp(0.0, 1.0)
                } else {
                    0.0
                };
                let remaining_secs = (next - now).num_seconds().max(0);

                ui.add(
                    egui::ProgressBar::new(progress)
                        .desired_width(180.0)
                        .text(format!("Next: {remaining_secs}s"))
                        .animate(false),
                );
            }
            _ if state.geojson_promise.is_some() => {
                // Show an indeterminate bar while the initial GeoJSON loads.
                ui.add(
                    egui::ProgressBar::new(0.0)
                        .desired_width(180.0)
                        .text("Loading cameras…")
                        .animate(false),
                );
            }
            _ => {}
        }

        // ── Camera count + settings toggle (right-aligned) ───────────────────
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            let settings_label = if state.settings_open {
                "Hide Settings"
            } else {
                "⚙ Settings"
            };
            let settings_response = ui.button(settings_label);
            if settings_response.clicked() {
                state.settings_open = !state.settings_open;
                if !state.settings_open {
                    // Revert any unapplied changes when the panel is closed.
                    state.pending_config = state.config.clone();
                }
                settings_response.surrender_focus();
            }

            ui.add_space(8.0);

            let n_districts = state.config.selected_districts.len();
            ui.label(format!(
                "{} cameras  |  {} district(s) selected",
                state.cameras.len(),
                n_districts
            ));
        });
    });
}
