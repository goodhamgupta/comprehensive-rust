// compiler can derive number of inbuilt traits

#[derive(Debug, Clone, PartialEq, Eq, Default)]
struct Player {
    name: String,
    strength: u8,
    hit_points: u8,
}

fn main() {
    let p1 = Player::default();
    let p2 = p1.clone();
    let result = if p1 == p2 { "yes" } else { "no" };

    println!(
        "Is {:?} equal to {:?}? \n The answer is {}!",
        &p1, &p2, result
    );
}
