// closures or lambda expressions have type that cannot be named
// They implement special Fn, FnMut and FnOnce traits

fn apply_with_log(mut func: impl FnMut(i32) -> i32, input: i32) -> i32 {
    println!("calling func on {input}");
    func(input)
}

fn main() {
    let mut add_3 = |x| x + 3;
    let mut mul_5 = |x| x * 5;
    println!("add_3: {}", apply_with_log(add_3, 10));
    println!("mul_5: {}", apply_with_log(mul_5, 10));
}
