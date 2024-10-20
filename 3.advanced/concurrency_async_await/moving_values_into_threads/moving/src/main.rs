// Fix the code to make it compile.

use std::thread;

fn main() {
    let msg = "Hello from spawned thread".to_owned();
    let handle = thread::spawn(move || {
        println!("{msg}");
    });
    handle.join().unwrap();
    println!("Hello from main thread");
}
