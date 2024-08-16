// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use app::write_atis;
use audio::play_audio;
use log::info;
use settings::{check_settings_file, read_settings, write_settings};
use tauri::{AppHandle, Listener};
use tauri_plugin_log::{Target, TargetKind};
use util::{is_vatis_running, open_vatis};

fn setup(app_handle: &AppHandle) {
    info!(
        "D-ATIS to vATIS started version {}",
        app_handle.config().version.as_ref().unwrap()
    );

    check_settings_file(&app_handle).unwrap();
}

fn main() {
    tauri::Builder::default()
        .setup(|app: &mut tauri::App| {
            let handle = app.handle().clone();
            setup(&handle);

            app.listen("new-codes", move |_event| {
                play_audio(&handle);
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
            write_settings,
            read_settings,
            write_atis,
            is_vatis_running,
            open_vatis
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
