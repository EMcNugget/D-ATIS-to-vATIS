// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use log::info;
use tauri::App;

mod app;
mod consts;
mod structs;
mod util;

fn setup(app: &mut App) {
    info!(
        "D-ATIS to vATIS started version {}",
        app.handle().config().version.as_ref().unwrap()
    );

    crate::util::settings::check_settings_file();
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
            crate::app::app::write_atis,
            crate::app::app::is_vatis_running,
            crate::app::app::open_vatis,
            crate::app::app::get_profiles,
            crate::app::audio::play_audio,
            crate::app::app::get_airports_in_profile,
            crate::util::settings::write_settings,
            crate::util::settings::read_settings,
            crate::util::assets::set_contractions,
            crate::util::assets::get_facility_config,
            crate::util::assets::get_contractions,
            crate::util::assets::get_runways,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
