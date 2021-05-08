use cli::parse_options;
use timers::{ tomato, short_break, long_break };

mod cli;
mod timers; 
mod notify;

fn main () {
    let (start, sbreak, lbreak, watch) = parse_options();
    if watch {
        println!("will get the time");
    } else {
        if start {
            tomato();
        } if sbreak {
            short_break();
        } if lbreak {
            long_break();
        }
    }
    println!("end of funciton reached");
}
