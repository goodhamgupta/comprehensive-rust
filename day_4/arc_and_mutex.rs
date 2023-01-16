use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let mut v = Mutex::new(vec![10, 20, 30]);
    let handle = thread::spawn(move || {
        let v: &Mutex<Vec<i32>> = &v;
        let mut guard = v.lock().unwrap();
        guard.push(10);
    });
    // let guard = v.lock().unwrap().clone();
    // guard.push(1000);
    handle.join().unwrap();
    println!("v: {v:?}");
}
