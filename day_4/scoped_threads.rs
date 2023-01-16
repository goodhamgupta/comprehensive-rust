use std::thread;

// normal threads cannot borrow from their environment
// fn main() {
//     let s = String::from("Hello");
//
//     thread::spawn(|| {
//         println!("length: {}", s.len());
//     });
// }
// scoped threads can borrow

fn main() {
    let s = String::from("Hello");

    thread::scope(|scope| {
        for i in 1..500000 {
            scope.spawn(|| {
                println!("Length: {}", s.len());
            });
        }
    });
}
