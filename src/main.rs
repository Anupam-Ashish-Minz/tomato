use cli::parse_options;
use timers::{ tomato, short_break, long_break };
use sound::play_sound;

mod cli;
mod timers; 
mod notify;
mod sound;

fn main () {
    let (start, sbreak, lbreak, watch) = parse_options();
    if watch {
        println!("will get the time");
    } else {
        if start {
            tomato();
            play_sound();
            println!("work time ended");
        } if sbreak {
            short_break();
            play_sound();
            println!("short break ended");
        } if lbreak {
            long_break();
            play_sound();
            println!("long break ended");
        }
    }
}
