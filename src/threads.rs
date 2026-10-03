use std::{
    sync::{Arc, Mutex},
    thread::{self, sleep},
    time::Duration,
};
pub fn main() {
    shared_counter();
}

fn shared_counter() {
    let mut handles = vec![];
    let shared_counter = Arc::new(Mutex::new(0u32));
    for _ in 0..10 {
        let counter: Arc<Mutex<u32>> = Arc::clone(&shared_counter);
        let handle = thread::spawn(move || {
            for _ in 0..1000 {
                let mut num = counter.lock().unwrap();
                *num += 1
            }
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    println!("{:?}", *shared_counter.lock().unwrap());
}

fn spawn_threads() {
    let mut handles = vec![];
    for i in 0..4 {
        let duration = Duration::new(i + 1, 0);
        let handle = thread::spawn(move || {
            sleep(duration);
            println!("Hello from thread {} ", i + 1);
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
