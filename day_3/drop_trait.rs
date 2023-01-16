// these values can specify code to run when they go out of scope
// i.e destructors

struct Droppable {
    name: &'static str,
}

impl Drop for Droppable {
    fn drop(&mut self) {
        println!("dropping {}", self.name);
    }
}

fn main() {
    let a = Droppable { name: "a" };
    {
        let b = Droppable { name: "b" };
        {
            let c = Droppable { name: "c" };
            let d = Droppable { name: "d" };
            println!("Exiting block b");
        }
        println!("Exiting block a");
    }
    drop(a);
    println!("exiting main");
}
