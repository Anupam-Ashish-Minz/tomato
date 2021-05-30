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
        "audacious",            // Our application’s name
        Direction::Playback, // We want a playback stream
        None,                // Use the default device
        "Music",             // Description of our stream
        &spec,               // Our sample format
        None,                // Use default channel map
        None                 // Use default buffering attributes
    ).unwrap();

    let mut buf = [0; 1024];
    let out = s.read(&mut buf);

    match out {
        Ok(_) => {},
        Err(e) => println!("Error {}", e)
    }
}

#[cfg(test)]
mod tests {
    use super::*; 

    #[test]
    fn test_sound() {
        let spec = Spec {
            format: Format::S16NE,
            channels: 2,
            rate: 44100,
        };
        assert!(spec.is_valid());
    }
}
