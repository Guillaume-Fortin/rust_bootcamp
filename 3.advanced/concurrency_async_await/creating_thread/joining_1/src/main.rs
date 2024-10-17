// Modify the code to ensure that main thread waits for other threads to finish.

use std::thread;

fn main() {
    // print hello 10 times from spawned thread
    let handle = thread::spawn(|| {
        for i in 0..10 {
            println!("{i} Hello from spawned thread!")
        }
    });

    // print hello 10 times from main thread
    for i in 0..10 {
        println!("{i} Hello from main thread!");
    }

    handle.join().unwrap();
}
