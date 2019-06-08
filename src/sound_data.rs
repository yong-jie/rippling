use rodio::Decoder;

use std::fs::File;
use std::io::{Cursor, Read, Result};
use std::sync::Arc;

pub struct SoundData(Arc<Vec<u8>>);

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
