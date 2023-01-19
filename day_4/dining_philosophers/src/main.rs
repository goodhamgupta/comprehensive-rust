#[allow(dead_code, unused_imports, unused_variables)]
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

struct Fork;

struct Philosopher {
    name: String,
    left_fork: Arc<Mutex<Fork>>,
    right_fork: Arc<Mutex<Fork>>,
    thoughts: Sender<String>,
}

impl Philosopher {
    fn think(&self) {
        self.thoughts
            .send(format!("Eureka! {} has a new idea!", &self.name))
            .unwrap()
    }

    fn eat(&self) {
        // pickup forks
        println!("{} is eating", &self.name);
        thread::sleep(Duration::from_millis(10));
    }
}

static PHILOSOPHERS: &[&str] = &["Socrates", "Plato", "Aristotle", "Thales", "Pythagoras"];

fn main() {
    // create forks
    let (send, recv): (Sender<String>, Receiver<String>) = mpsc::channel();
    let left_fork = Arc::new(Mutex::new(Fork));
    let right_fork = Arc::new(Mutex::new(Fork));
    // let completed = Arc::new(Mutex::new(0));
    let mut philosophers: Vec<Philosopher> = vec![];
    for name in PHILOSOPHERS {
        philosophers.push(Philosopher {
            name: String::from(*name),
            left_fork: left_fork.clone(),
            right_fork: right_fork.clone(),
            thoughts: send.clone(),
        });
    }
    for philosopher in philosophers {
        // let mut completed = Arc::clone(&completed);
        thread::spawn(move || {
            let left_fork = philosopher.left_fork.lock();
            let right_fork = philosopher.right_fork.lock();
            // let completed = completed.lock();
            if left_fork.is_ok() & right_fork.is_ok() {
                philosopher.eat();
            } else {
                println!("sending");
                philosopher.think();
            }
        });
    }
    for received_msg in recv {
        println!("Got: {}", received_msg)
    }
}
