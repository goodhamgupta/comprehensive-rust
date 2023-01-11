// mod lets us namespace types and functions
mod foo {
    pub fn do_something() {
        println!("in the foo module");
    }
}

mod bar {
    pub fn do_something() {
        println!("in the bar module");
    }
}

fn main() {
    foo::do_something();
    bar::do_something();
}
