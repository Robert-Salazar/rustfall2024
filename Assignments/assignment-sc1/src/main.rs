use std::thread;

fn main() {
    // Vector to hold the thread handles
    let mut handles = vec![];

    for i in 1..=3 {
        let handle = thread::spawn(move || {
            println!("Thread {}", i);
        });
        handles.push(handle);
    }

    // Wait for all threads to finish
    for handle in handles {
        handle.join().expect("Thread failed to join.");
    }

    println!("All threads completed.");
}
