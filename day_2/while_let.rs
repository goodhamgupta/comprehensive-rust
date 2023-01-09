fn main() {
    let v = vec![10, 20, 30];
    let mut iter = v.into_iter();

    // iter will return Option<i32> on every call to next
    // while let lets us keep iterationg through all items
    while let Some(x) = iter.next() {
        println!("x: {x}");
    }
}
