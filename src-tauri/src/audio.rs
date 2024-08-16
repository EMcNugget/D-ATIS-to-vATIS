use log::info;
use rodio::{source::Source, Decoder, OutputStream};
use tauri::AppHandle;

use crate::util::get_resource;

pub fn play_audio(app_handle: &AppHandle) -> () {
    let file = get_resource(app_handle, "AtisUpdate.wav");

    match file {
        Ok(file) => {
            let (_stream, stream_handle) = OutputStream::try_default().unwrap();
            let source = Decoder::new(file).unwrap();
            stream_handle.play_raw(source.convert_samples()).unwrap();
            info!("Playing ATIS update sound");
            Ok(())
        }
        Err(e) => Err({
            error!("Failed to play audio: {}", e);
            e
        }),
    }
    std::thread::sleep(std::time::Duration::from_secs(2));
}
