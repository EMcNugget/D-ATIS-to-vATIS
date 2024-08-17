// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use log::info;
use tauri::{AppHandle, Listener};
use tauri_plugin_log::{Target, TargetKind};

fn setup(app_handle: &AppHandle) {
    info!(
        "D-ATIS to vATIS started version {}",
        app_handle.config().version.as_ref().unwrap()
    );

    d_atis_to_vatis::settings::check_settings_file(&app_handle);
}

fn main() {
    tauri::Builder::default()
        .setup(|app: &mut tauri::App| {
            let handle = app.handle().clone();
            setup(&handle);

            app.listen("new-codes", move |_event| {
                d_atis_to_vatis::audio::play_audio(&handle);
            });

            Ok(())
        })
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_dialog::init())
        .plugin(
            tauri_plugin_log::Builder::new()
                .targets([
                    Target::new(TargetKind::Stdout),
                    Target::new(TargetKind::LogDir {
                        file_name: Some("log".to_string()),
                    }),
                ])
                .timezone_strategy(tauri_plugin_log::TimezoneStrategy::UseLocal)
                .rotation_strategy(tauri_plugin_log::RotationStrategy::KeepOne)
                .build(),
        )
        .invoke_handler(tauri::generate_handler![
            d_atis_to_vatis::settings::write_settings,
            d_atis_to_vatis::settings::read_settings,
            d_atis_to_vatis::app::write_atis,
            d_atis_to_vatis::util::is_vatis_running,
            d_atis_to_vatis::util::open_vatis,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
