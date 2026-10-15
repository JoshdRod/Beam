use rodio::{Decoder, MixerDeviceSink, source::Source};
use std::env;
use std::fs::File;
use std::io::BufReader;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path: &String = &args[1];
    println!("{}", file_path);

    let contents = File::open(file_path).expect("Read file");

    // A buffer is a collection of read bytes, that's stored in main memory.
    // Without using a buffer, the data is actually..
    let contents_buffer = BufReader::new(contents);

    // Get an OS-Sink handle to the default physical sound device.
    // Note that the playback stops when the sink_handle is dropped.
    let sink_handle =
        rodio::DeviceSinkBuilder::open_default_sink().expect("open default audio stream");

    // Note that the playback stops when the player is dropped
    let player = rodio::play(&sink_handle.mixer(), contents_buffer).unwrap();

    // The sound plays in a separate audio thread,
    // so we need to keep the main thread alive while it's playing.
    std::thread::sleep(std::time::Duration::from_secs(5));
}
