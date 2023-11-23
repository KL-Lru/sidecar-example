use std::{time::Duration, thread::sleep};

fn main() {
    println!("Main container wake up!");

    sleep(Duration::from_secs(10));

    println!("Main container is shutting down!");
}
