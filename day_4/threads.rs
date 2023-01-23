use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("Count in thread: {i}");
            thread::sleep(Duration::from_millis(5));
        }
    }); // daemon threads created. main will not wait for them to finish.
        // the sleep time.
    for i in 1..5 {
        println!("main thread: {i}");
        thread::sleep(Duration::from_millis(5));
    }
    let result = handle.join(); // this will wait for all threads
                                // to complete before exiting
}

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}