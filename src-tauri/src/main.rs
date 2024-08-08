// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod contraction;
mod settings;
mod structs;
mod util;

use app::write_atis;
use log::info;
use settings::{create_settings_file, read_settings, write_settings};
use tauri::AppHandle;
use tauri_plugin_log::{Target, TargetKind};
use util::is_vatis_running;

fn setup(app_handle: AppHandle) {
    info!(
        "D-ATIS to vATIS started version {}",
        app_handle.config().version.as_ref().unwrap()
    );

    create_settings_file(app_handle).unwrap();
}

fn main() {
    tauri::Builder::default()
        .setup(|app: &mut tauri::App| Ok(setup(app.handle().clone())))
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_dialog::init())
        .plugin(
            tauri_plugin_log::Builder::new()
                .targets([
                    Target::new(TargetKind::Stdout),
                    Target::new(TargetKind::LogDir { file_name: None }),
                    Target::new(TargetKind::Webview),
                ])
                .build(),
        )
        .invoke_handler(tauri::generate_handler![
            write_settings,
            read_settings,
            write_atis,
            is_vatis_running
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
