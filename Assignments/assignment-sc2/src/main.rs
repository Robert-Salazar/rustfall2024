use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    // Shared counter wrapped in Arc and Mutex
    let counter = Arc::new(Mutex::new(0));

    // Vector to hold the thread handles
    let mut handles = vec![];

    for _ in 0..5 {
        // Clone the Arc to share it across threads
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            for _ in 0..10 {
                let mut num = counter.lock().unwrap();
                *num += 1;
            }
        });
        handles.push(handle);
    }

    // Wait for all threads to finish
    for handle in handles {
        handle.join().expect("Thread failed to join.");
    }

    // Print the final value of the counter
    println!("Final counter value: {}", *counter.lock().unwrap());
}
