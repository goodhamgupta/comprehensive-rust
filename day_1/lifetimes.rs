#[derive(Debug)]
struct Point(i32, i32);

// Read &'a Point as “a borrowed Point which is valid for at least the lifetime a”.
fn left_most<'a>(p1: &'a Point, p2: &'a Point) -> &'a Point {
    if p1.0 < p2.0 {
        p1
    } else {
        p2
    }
}

fn main() {
    let p1: Point = Point(10, 10);
    let p2: Point = Point(0, 20);
    let p3: &Point = left_most(&p1, &p2);
    println!("left-most point: {:?}", p3);
}
