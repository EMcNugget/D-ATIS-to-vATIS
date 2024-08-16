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

    D_ATIS_to_VATIS::settings::check_settings_file(&app_handle).unwrap();
}

fn main() {
    tauri::Builder::default()
        .setup(|app: &mut tauri::App| {
            let handle = app.handle().clone();
            setup(&handle);

            app.listen("new-codes", move |_event| {
                D_ATIS_to_VATIS::audio::play_audio(&handle);
            });

            Ok(())
        })
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
            D_ATIS_to_VATIS::settings::write_settings,
            D_ATIS_to_VATIS::settings::read_settings,
            D_ATIS_to_VATIS::app::write_atis,
            D_ATIS_to_VATIS::util::is_vatis_running,
            D_ATIS_to_VATIS::util::open_vatis
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
