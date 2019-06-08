extern crate rodio;

use rodio::decoder::Decoder;
use std::fs::File;
use std::io::BufReader;

pub fn play_music(filename: &str) -> rodio::Sink {
    let device = rodio::default_output_device().unwrap();
    let sink = rodio::Sink::new(&device);
    let music_source = decode_file(filename);
    sink.append(music_source);
    sink
}

fn decode_file(filename: &str) -> Decoder<BufReader<File>> {
    let file: File = File::open(filename).unwrap();
    let buf_reader: BufReader<File> = BufReader::new(file);
    let source = Decoder::new(buf_reader).unwrap();
    source
}

