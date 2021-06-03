use std::fs;
use psimple::Simple;
use pulse::stream::Direction;
use pulse::sample::{Spec, Format};

pub fn play_something() {
    let spec = Spec {
        format: Format::S16NE,
        channels: 2,
        rate: 44100,
    };
    assert!(spec.is_valid());

    let s = Simple::new(
        None,                // Use the default server
        "random",            // Our application’s name
        Direction::Playback, // We want a playback stream
        None,                // Use the default device
        "Music",             // Description of our stream
        &spec,               // Our sample format
        None,                // Use default channel map
        None                 // Use default buffering attributes
    ).unwrap();

    let music_data = fs::read("assets/swiftly-610.mp3")
        .expect("failed to read music file");

    let out = s.write(&music_data);
    match out {
        Ok(data) => println!("ok {:?}", data),
        Err(e) => println!("error {}", e)
    }
}

#[cfg(test)]
mod tests {
    use super::*; 
}
