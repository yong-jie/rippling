
use rodio::{Device, Sink, Source};
use std::cell::RefCell;
use std::time::Duration;
use std::sync::{Arc, Mutex};
use crate::sound_data;

/// A playable audio.
pub trait Audio {
    fn play(&self);
}

/// Music is designed to be played with no overlaps.
/// This means that `play()` should only work when
/// the music is not currently playing.
pub struct Music {
    // We store a Device so we can rebuild Sinks to
    // implement stopping. Sinks need to be rebuilt
    // because rodio's stop function actually renders
    // sinks unusable.
    device: Device,
    sink: RefCell<Sink>,
    sound_data: sound_data::SoundData,
    duration: RefCell::<Arc<Mutex<Duration>>>
}

impl Music {
    /// Music is by default not started during instantiation.
    pub fn new(
        device: &Device,
        sound_data: sound_data::SoundData) -> Music {
        let duration = RefCell::new(Arc::new(Mutex::new(Duration::from_secs(0))));
        Music {
            device: device.to_owned(),
            sink: RefCell::new(Sink::new(&device)),
            sound_data,
            duration
        }
    }

    /// Pauses the Music. Can be resumed with play().
    pub fn pause(&self) {
        self.sink.borrow().pause();
    }

    /// Stops the Music. Subsequent calls to play() will
    /// start the Music from scratch.
    pub fn stop(&self) {
        self.sink.replace(Sink::new(&self.device));
        self.duration.replace(Arc::new(Mutex::new(Duration::from_secs(0))));
    }
}

impl Audio for Music {
    fn play(&self) {
        if self.sink.borrow().empty() {
            // This is the first time playing.
            let decoder = self.sound_data.decoder();

            // Make this sound pausable.
            let pausable = decoder.pausable(false);

            // Enable tracking of time elapsed
            let elapsed = pausable.elapsed(self.duration.borrow().clone());

            self.sink.borrow().append(elapsed);
        } else if self.sink.borrow().is_paused() {
            self.sink.borrow().play();
        }
    }
}

