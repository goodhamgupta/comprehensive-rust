// mutex ensures mutual exclusion and allows mutable access to T behind a read-only interface

// mutex is considered posioned if any of the thread holding the mutex panics. This means that
// the mutex value is no longer reliable, and you should be careful before using it.
use std::sync::Mutex;

fn main() {
    let v: Mutex<Vec<i32>> = Mutex::new(vec![10, 20, 30]);

    println!("v: {:?}", v.lock().unwrap());

    {
        let v: &Mutex<Vec<i32>> = &v;
        let mut guard = v.lock().unwrap();
        guard.push(40);
    }
    println!("v: {:?}", v.lock().unwrap());
}
