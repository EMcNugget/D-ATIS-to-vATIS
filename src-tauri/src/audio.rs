use log::info;
use rodio::{source::Source, Decoder, OutputStream};
use tauri::{AppHandle, Manager, UserAttentionType};

use crate::util::get_resource;

pub fn play_audio(app_handle: &AppHandle) -> () {
    let file = get_resource(app_handle, "AtisUpdate.wav").unwrap();

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
