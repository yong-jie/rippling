use rodio::Decoder;

use std::fs::File;
use std::io::{Cursor, Read, Result};
use std::sync::Arc;

/// This abstraction exists because a rodio sink requires a new Decoder
/// object to be fed into it whenever an audio is played. Creating a
/// normal decoder with fresh new binary sound data is extremely
/// IO-inefficient, and as such we load the sound file into a buffer
/// first use cursors to generate new Decoder objects for the sink.
pub struct SoundData(Arc<Vec<u8>>);

/// For use with io::Cursor.
impl AsRef<[u8]> for SoundData {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}
 
impl SoundData {
    pub fn load(filename: &str) -> Result<SoundData> {
        let mut file = File::open(filename)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;
        Ok(SoundData(Arc::new(buffer)))
    }

    pub fn cursor(&self) -> Cursor<SoundData> {
        Cursor::new(SoundData(self.0.clone()))
    }

    pub fn decoder(&self) -> Decoder<Cursor<SoundData>> {
        Decoder::new(self.cursor()).unwrap()
    }
}
