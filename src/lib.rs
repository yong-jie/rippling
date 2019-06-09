extern crate rodio;

use crate::audio::Audio;

pub use rodio::{
    default_input_device, default_output_device, devices, input_devices, output_devices, Device,
};

pub mod audio;
pub mod sound_data;

pub fn play_music(filename: &str) -> audio::Music {
    let music = get_music(filename);
    music.play();
    music
}

/// Get a stopped music from filename.
///
/// Uses default device.
pub fn get_music(filename: &str) -> audio::Music {
    let device = rodio::default_output_device().unwrap();
    let sink = rodio::Sink::new(&device);
    let sound_data = sound_data::SoundData::load(filename).unwrap();
    audio::Music::new(sink, sound_data)
}

/// Get a stopped music from filename, using specified Device.
pub fn get_music_with_device(filename: &str, device: &Device) -> audio::Music {
    let sink = rodio::Sink::new(&device);
    let sound_data = sound_data::SoundData::load(filename).unwrap();
    audio::Music::new(sink, sound_data)
}
