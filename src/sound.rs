use std::fs::File;
use std::io::BufReader;
use rodio::{Decoder, OutputStream, source::Source};

pub fn play_something() {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let file = BufReader::new(File::open("assets/swiftly-610.mp3").unwrap());
    let source = Decoder::new(file).unwrap();
    let res = stream_handle.play_raw(source.convert_samples());
    std::thread::sleep(std::time::Duration::from_secs(5));

    match res {
        Ok(_) => {},
        Err(e) => println!("error in sound {}", e)
    }
}
