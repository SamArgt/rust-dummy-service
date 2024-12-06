use std::thread;
use std::time::Duration;

fn main() {
    loop {
        println!("I am alive");
        thread::sleep(Duration::from_secs(60));
    }
}