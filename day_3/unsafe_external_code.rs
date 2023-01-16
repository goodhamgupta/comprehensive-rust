// functions from other languages might violate rust guarantees.
// Use unsafe

extern "C" {
    fn abs(input: i32) -> i32;
}

fn main() {
    unsafe {
        println!("Abs value of -3 according to C: {}", abs(-3));
    }
}
