fn main() {
    let v = vec![10, 20, 30];
    // for will call into_iter() automatically on the expression and then iterate over it
    for x in v {
        println!("x: {x}");
    }
}
