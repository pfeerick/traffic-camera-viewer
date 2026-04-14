// On Windows release builds, suppress the background console window.
// Debug builds keep the console so env_logger output is visible.
#![cfg_attr(
    all(target_os = "windows", not(debug_assertions)),
    windows_subsystem = "windows"
)]

mod app;
mod camera;
mod config;
mod fetcher;
mod ui;

use app::AppState;

fn main() {
    env_logger::init();

    // Build the Tokio runtime BEFORE handing control to eframe.
    // rt.enter() installs the runtime context on this thread, making
    // tokio::runtime::Handle::current() available everywhere — which is what
    // poll_promise::Promise::spawn_async() relies on internally.
    let rt = tokio::runtime::Runtime::new().expect("Failed to create Tokio runtime");
    let _guard = rt.enter(); // keeps context alive until main() returns

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title(concat!(
                "QLD Traffic Camera Viewer v",
                env!("CARGO_PKG_VERSION")
            ))
            .with_inner_size([1200.0_f32, 800.0_f32])
            .with_min_inner_size([600.0_f32, 400.0_f32]),
        ..Default::default()
    };

    eframe::run_native(
        "traffic-camera-viewer",
        native_options,
        Box::new(|cc| Ok(Box::new(AppState::new(cc)))),
    )
    .expect("eframe failed to start");
}
