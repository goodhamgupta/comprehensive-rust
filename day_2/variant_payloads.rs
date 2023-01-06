// use match to extract data from each variant. Similar to elixir pattern match
enum WebEvent {
    PageLoad,                 // variant without payload
    KeyPress(char),           // tuple struct variant
    Click { x: i64, y: i64 }, // Full struct variant
}

fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("page loaded"),
        WebEvent::KeyPress(c) => println!("pressed {c}"),
        WebEvent::Click { x, y } => println!("x = {x} y = {y}"),
    }
}

fn main() {
    let load = WebEvent::PageLoad;
    let press = WebEvent::KeyPress('x');
    let click = WebEvent::Click { x: 20, y: 10 };

    inspect(load);
    inspect(press);
    inspect(click);
}
