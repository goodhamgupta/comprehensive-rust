struct Point(u8, u8);

// fn main() {
//     let coordinate = Point(10, 11);
//     println!("x is {} and y is {}", coordinate.0, coordinate.1);
// }

// generally used for single field wrapper aka newtypes

struct PoundOfForce(f64);
struct Newtons(f64);

fn compute_thruster_force() -> PoundOfForce {
    PoundOfForce(10)
}

fn set_thruster_force(force: Newtons) {}

fn main() {
    let force = compute_thruster_force();
    set_thruster_force(force);
}
