use notify_rust::{ Notification, Timeout };

pub fn send_notifications(summary: &str, body: &str) {
    Notification::new()
    .summary(summary)
    .body(body)
    .timeout(Timeout::Milliseconds(6000)) 
    .show().unwrap();
}

