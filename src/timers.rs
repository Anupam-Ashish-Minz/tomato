use std::thread::sleep;
use std::time::Duration;

use super::notify::send_notifications;

pub fn tomato() {
    sleep(Duration::from_secs(60*25));
    send_notifications(
        "🍅 tomato",
        "🍅🍅🍅🍅🍅🍅🍅"
    );
}

pub fn short_break() {
    sleep(Duration::from_secs(60*5));
    send_notifications(
        "🌅 break ends",
        "short break has ended"
    );
}

pub fn long_break() {
    sleep(Duration::from_secs(60*10));
    send_notifications(
        "⚠️ peacefull days have ended",
        "long break has ended"
    );
}

