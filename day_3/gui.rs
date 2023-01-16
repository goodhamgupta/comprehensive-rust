// Implementation of simple gui library

#![allow(unused_imports, unused_variables, dead_code)]

pub trait Widget {
    fn width(&self) -> usize;

    fn draw_into(&self, buffer: &mut dyn std::fmt::Write);

    fn draw(&self) {
        let mut buffer = String::new();
        self.draw_into(&mut buffer);
        println!("{}", &buffer);
    }
}

pub struct Label {
    label: String,
}

impl Label {
    fn new(label: &str) -> Label {
        Label {
            label: label.to_owned(),
        }
    }
}

pub struct Button {
    label: Label,
    callback: Box<dyn FnMut()>,
}

impl Button {
    fn new(label: &str, callback: Box<dyn FnMut()>) -> Button {
        Button {
            label: Label::new(label),
            callback,
        }
    }
}

pub struct Window {
    title: String,
    widgets: Vec<Box<dyn Widget>>,
}

impl Window {
    fn new(title: &str) -> Window {
        Window {
            title: title.to_owned(),
            widgets: Vec::new(),
        }
    }

    fn add_widget(&mut self, widget: Box<dyn Widget>) {
        self.widgets.push(widget)
    }
}

impl Widget for Label {
    fn width(&self) -> usize {
        self.label.len()
    }

    fn draw_into(&self, buffer: &mut dyn std::fmt::Write) {
        buffer.write_str(&self.label).unwrap();
        buffer.write_str("\n").unwrap();
    }
}

impl Widget for Button {
    fn width(&self) -> usize {
        unimplemented!()
    }

    fn draw_into(&self, buffer: &mut dyn std::fmt::Write) {
        buffer.write_str("+-----+\n").unwrap();
        buffer.write_str(&self.label.label).unwrap();
        buffer.write_str("\n+-----+\n").unwrap();
    }
}

impl Widget for Window {
    fn width(&self) -> usize {
        unimplemented!()
    }

    fn draw_into(&self, buffer: &mut dyn std::fmt::Write) {
        buffer.write_str("=======\n").unwrap();
        buffer.write_str(&self.title).unwrap();
        buffer.write_str("\n=======\n").unwrap();
        for widget in &self.widgets {
            widget.draw_into(buffer)
        }
    }
}

fn main() {
    let mut window = Window::new("Rust GUI demo 1.23");
    window.add_widget(Box::new(Label::new("this is a small text gui demo.")));
    window.add_widget(Box::new(Button::new(
        "Click me!",
        Box::new(|| println!("You clicked the button")),
    )));
    window.draw();
}
