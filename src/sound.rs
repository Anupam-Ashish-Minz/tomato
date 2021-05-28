use libpulse_simple_binding::Simple;
use libpulse_binding::stream::Direction;
use libpulse_binding::sample::{ Spec, Format };

pub fn play_something() {
    let spec = Spec {
        format: Format::S16NE,
        channels: 2,
        rate: 44100,
    };
    assert!(spec.is_valid());

    let s = Simple::new(
        None,                // Use the default server
        "FooApp",            // Our application’s name
        Direction::Playback, // We want a playback stream
        None,                // Use the default device
        "Music",             // Description of our stream
        &spec,               // Our sample format
        None,                // Use default channel map
        None                 // Use default buffering attributes
    ).unwrap();

    let mut data = [0; 1024];
    let res = s.read(&mut data);

    match res {
        Ok(_) => {
        },
        Err(err) => {
            println!("error {}", err);
        }
    };
}
