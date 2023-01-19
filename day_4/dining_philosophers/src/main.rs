use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

struct Fork;

struct Philosopher {
    name: String,
    left_fork: Fork,
    right_fork: Fork,
    thoughts: vec![],
}

impl Philosopher {
    fn think(&self) {}
}

fn main() {}
