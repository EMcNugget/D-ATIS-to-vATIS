use log::info;
use rodio::{source::Source, Decoder, OutputStream};
use std::fs::File;
use tauri::{AppHandle, Manager, UserAttentionType};

#[tauri::command]
pub fn play_audio(app_handle: AppHandle) -> () {
    let file = File::open("../assets/AtisUpdate.wav").unwrap();

    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let source = Decoder::new(file).unwrap();
    stream_handle.play_raw(source.convert_samples()).unwrap();

    std::thread::sleep(std::time::Duration::from_secs(2));

    info!("ATIS Update audio played");

    app_handle
        .get_webview_window("main")
        .unwrap()
        .request_user_attention(Some(UserAttentionType::Informational))
        .unwrap();
}
