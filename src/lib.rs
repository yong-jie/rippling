extern crate rodio;

use std::io::BufReader;

pub fn play_music(filename: &str) -> rodio::Sink {
    let device = rodio::default_output_device().unwrap();
    let sink = rodio::Sink::new(&device);
    let file = std::fs::File::open(filename).unwrap();
    let source = rodio::Decoder::new(BufReader::new(file)).unwrap();
    sink.append(source);
    sink
}

