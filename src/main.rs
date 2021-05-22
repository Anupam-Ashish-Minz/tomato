use cli::parse_options;
use timers::{ tomato, short_break, long_break };
use sound::play_something;

mod cli;
mod timers; 
mod notify;
mod sound;

fn main () {
    //let (start, sbreak, lbreak, watch) = parse_options();
    //if watch {
    //    println!("will get the time");
    //} else {
    //    if start {
    //        tomato();
    //    } if sbreak {
    //        short_break();
    //    } if lbreak {
    //        long_break();
    //    }
    //}
    //play_something();
    //println!("end of funciton reached");
    println!("start");
    play_something();
    println!("end");
}
