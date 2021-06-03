use clap::{App, Arg};

pub fn parse_options() -> (bool, bool, bool, bool) {
    let matchs = App::new("tomato-timer")
        .about("based of the pomodoro timer concept")
        .version("0.1")
        .author("Anupam Ashish Minz")
        .arg(Arg::with_name("start")
            .short("t")
            .long("tomato")
            .help("starts the tomato clock 25min")
            .takes_value(false))
        .arg(Arg::with_name("short_break")
            .short("b")
            .long("short_break")
            .help("starts the short break 5min")
            .takes_value(false))
        .arg(Arg::with_name("long_break")
            .short("l")
            .long("long_break")
            .help("starts the long break 10min")
            .takes_value(false))
        .arg(Arg::with_name("watch")
            .short("w")
            .long("watch")
            .help("gets the time remaining")
            .takes_value(false))
        .get_matches();
    let start = matchs.is_present("start");
    let sbreak = matchs.is_present("short_break");
    let lbreak = matchs.is_present("long_break");
    let watch = matchs.is_present("watch");
    return (start, sbreak, lbreak, watch);
}
