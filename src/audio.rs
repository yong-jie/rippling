
use rodio::Sink;
use crate::sound_data;

/// A playable audio.
pub trait Audio {
    fn play(&self);
}

/// Music is designed to be played with no overlaps.
/// This means that `play()` should only work when
/// the music is not currently playing.
pub struct Music {
    sink: Sink,
    sound_data: sound_data::SoundData
}

impl Music {
    /// Music is by default not started during instantiation.
    pub fn new(
        sink: Sink,
        sound_data: sound_data::SoundData) -> Music {
        Music {
            sink,
            sound_data
        }
    }
}

impl Audio for Music {
    fn play(&self) {
        if self.sink.empty() {
            // This is the first time playing.
            self.sink.append(self.sound_data.decoder());
        } else if self.sink.is_paused() {
            self.sink.play();
        }
    }
}

