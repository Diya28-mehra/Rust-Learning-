use chrono::{Local,Utc};
fn main() {
    let now = Utc::now();
    println!("Current UTC time: {}", now);

    let formatted = now.format("%Y-%m-%d %H:%M:%S").to_string();
    println!("Formatted UTC time: {}", formatted);

    let local_now = Local::now();
    println!("Current Local time: {}", local_now);
}
