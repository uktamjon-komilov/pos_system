//! Jowi POS desktop UI. Startup only; see `app.rs` for the wiring.

// Hides the console window in release builds on Windows.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod controllers;
mod domain;
#[cfg(test)]
mod layout_snapshots;
mod mappers;
mod repository;
mod ui;

fn main() -> Result<(), slint::PlatformError> {
    app::run()
}
