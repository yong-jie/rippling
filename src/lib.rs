extern crate rodio;

use crate::audio::Audio;

pub mod audio;
pub mod sound_data;

pub fn play_music(filename: &str) -> audio::Music {
    let device = rodio::default_output_device().unwrap();
    let sink = rodio::Sink::new(&device);
    let sound_data = sound_data::SoundData::load(filename).unwrap();
    let music = audio::Music::new(sink, sound_data);
    music.play();
    music
}
