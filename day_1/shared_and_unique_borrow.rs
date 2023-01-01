fn main() {
    let mut a: i32 = 10;
    // Multiple &T values allowed
    let b: &i32 = &a;
    println!("a: {a}");
    println!("b: {b}");

    {
        // ONLY 1 &mut value allowed at a time i.e only the code in a single scope can edit a given
        // variables
        let c: &mut i32 = &mut a;
        *c = 20;
        println!("c: {c}");
    }
}
