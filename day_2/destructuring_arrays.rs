fn main() {
    let triple = [1, -2, 3];
    println!("Tell me about {triple:?}");
    match triple {
        [0, y, z] => println!("x=0, y={y}, z={z}"),
        _ => println!("All ignored"),
    }
}
