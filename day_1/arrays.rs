fn main() {
    let array = [10, 20, 30];

    println!("array: {array:?}");

    for n in array {
        print!(" {n}");
    }

    for i in 0..3 {
        print!(" {}", array[i]);
    }
}
