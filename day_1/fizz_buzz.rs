fn main() {
    let num: i32 = 20;

    if num % 15 == 0 {
        println!("fizzbuzz");
    } else if num % 5 == 0 {
        println!("buzz");
    } else if num % 3 == 0 {
        println!("fizz");
    } else {
        println!("shampoo");
    }
}
