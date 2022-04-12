#![forbid(unsafe_code)]
#![cfg_attr(not(debug_assertions), deny(warnings))] // Forbid warnings in release builds
#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] //Hide console window in release builds on Windows, this blocks stdout.

use eframe::egui::CursorIcon::Default;

// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
fn main() {
    let app = chippie::TemplateApp::default();
    let native_options = eframe::NativeOptions {
        maximized: true,
        ..eframe::NativeOptions::default()
    };
    eframe::run_native(Box::new(app), native_options);
}
