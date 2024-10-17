// When main thread terminates, all the spawned threads also get terminated.
// Make the main thread sleep for half a second before terminating, to ensure that spawned thread gets enough time to complete.
// Do not join the spawned thread.

use std::{thread, time::Duration};

fn main() {
    thread::spawn(|| {
        println!("Count down from 10 to 1 🚀");
        for num in (1..=10).rev() {
            println!("Count down: {num}!");
        }
    });
    println!("Count up from 1 to 10...");
    for num in 1..=10 {
        println!("Count up: {num}");
    }

    thread::sleep(Duration::from_millis(250));
}
