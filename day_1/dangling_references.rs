fn main() {
    let ref_x: &i32;
    {
        let x: i32 = 10;
        ref_x = &x; // Borrow occurs here
    }
    println!("ref_x: {ref_x}");
}
