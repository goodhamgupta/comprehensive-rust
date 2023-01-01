struct Point(i32, i32);

fn main() {
    {
        let p = Point(3, 4);
        println!("x: {}", p.0);
    }
    // will not work as p is out-of-scope
    println!("y: {}", p.1);
}
