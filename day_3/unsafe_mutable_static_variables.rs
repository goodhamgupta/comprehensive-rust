// safe to read a immutable static variable

static HELLO_WORLD: &str = "Hello, world!";
// fn main() {
//     println!("name is: {}", HELLO_WORLD);
// }
// due to data races, unsafe to read and write mutable static variables

static mut COUNTER: u32 = 0;

fn add_to_counter(inc: u32) {
    unsafe { COUNTER += inc };
}
fn main() {
    add_to_counter(42);
    unsafe { println!("Counter: {}", COUNTER) };
}
