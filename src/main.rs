use std::thread::sleep;
use std::time::Duration;

use cli::parse_options;
use notify::send_notifications;

mod notify;
mod cli;

fn start_timer(sleeptime: Duration) {
    sleep(sleeptime);
    send_notifications(
        "🚓 start timer",
        "example"
    );
}

fn tomato() {
    sleep(Duration::from_millis(60*25));
    send_notifications(
        "🍅 tomato",
        "🍅🍅🍅🍅🍅🍅🍅"
    );
}

fn short_break() {
    sleep(Duration::from_millis(60*5));
    send_notifications(
        "🌅 break ends",
        "short break has ended"
    );
}

fn long_break() {
    sleep(Duration::from_millis(60*10));
    send_notifications(
        "⚠️ peacefull days have ended",
        "long break has ended"
    );
}



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
