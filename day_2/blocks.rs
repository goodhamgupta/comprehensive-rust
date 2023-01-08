// last line of block is returned and assigned to the variable
fn main() {
    let x = {
        let y = 10;
        println!("y: {y}");
        let z = {
            let w = { 3 + 4 };
            println!("w: {w}");
            y * w
        };
        println!("z: {z}");
        z - y
    };
    println!("x: {x}");
}
