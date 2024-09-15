// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use log::info;
use tauri::App;

mod app;
mod assets;
mod audio;
mod consts;
mod contraction;
mod settings;
mod structs;

fn setup(app: &mut App) {
    info!(
        "D-ATIS to vATIS started version {}",
        app.handle().config().version.as_ref().unwrap()
    );

    crate::settings::check_settings_file();
}

fn main() {
    tauri::Builder::default()
        .setup(|app| Ok(setup(app)))
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_dialog::init())
        .plugin(
            tauri_plugin_log::Builder::new()
                .timezone_strategy(tauri_plugin_log::TimezoneStrategy::UseLocal)
                .max_file_size(50_000)
                .rotation_strategy(tauri_plugin_log::RotationStrategy::KeepOne)
                .build(),
        )
        .invoke_handler(tauri::generate_handler![
            crate::settings::write_settings,
            crate::settings::read_settings,
            crate::app::write_atis,
            crate::app::is_vatis_running,
            crate::app::open_vatis,
            crate::app::get_profiles,
            crate::app::get_facility_config,
            crate::app::get_airports_in_profile,
            crate::contraction::set_contractions,
            crate::contraction::get_contractions,
            crate::audio::play_audio,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
