// generic code is turned into non-generic code based on call sites
// i.e zero cost abstraction

fn main() {
    let integer = Some(5);
    let float = Some(5.0);
}

// above behaves as if we wrote
// enum Option_i32 {
//     Some(i32),
//     None
// }
// enum Option_f64 {
//     Some(f64),
//     None
// }
// fn main() {
//     let integer = Option_i32::Some(5);
//     let float = Option_f64::Some(5.0);
// }
