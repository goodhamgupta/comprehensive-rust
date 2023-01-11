// box is an owned pointer to data on the heap
fn main() {
    let five = Box::new(10);
    println!("five: {}", *five);
}
