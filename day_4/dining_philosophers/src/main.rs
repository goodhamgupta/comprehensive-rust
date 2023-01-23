#[allow(dead_code, unused_imports, unused_variables, unused_import_braces)]
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender, SyncSender};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

#[derive(Debug)]
struct Fork;

#[derive(Debug, Clone)]
struct Philosopher {
    name: String,
    left_fork: Arc<Mutex<Fork>>,
    right_fork: Arc<Mutex<Fork>>,
    thoughts: SyncSender<String>,
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
        let left = self.left_fork.lock().unwrap();
        let right = self.right_fork.lock().unwrap();
        thread::sleep(Duration::from_millis(10));
        println!("{} is done eating", &self.name);
    }
}

static PHILOSOPHERS: &[&str] = &["Socrates", "Plato", "Aristotle", "Thales", "Pythagoras"];

fn main() {
    /*
    Key differences from my implementation:
    - Left and right forks locks were obtained as part of the eat function for each philosopher
    - No fixed object creation per philosopher. This explains why my logs only had 5 output statements. I created one object for each philosopher
    - Loop multiple times between eating and thinking. This was the reason why my program never terminated, as after one philosopher put down both forks
        there was no one left to pick them up, since everyone was thinking.
    - Breaking symmetry: Still not sure how switching the left and right fork in memomry help. Will come back to this later.
    */
    // create forks
    let (tx, rx): (SyncSender<String>, Receiver<String>) = mpsc::sync_channel(10);
    let forks = (0..PHILOSOPHERS.len())
        .map(|_| Arc::new(Mutex::new(Fork)))
        .collect::<Vec<_>>();
    for i in 0..forks.len() {
        let tx = tx.clone();
        let mut left_fork = forks[i].clone();
        let mut right_fork = forks[(i + 1) % forks.len()].clone();

        // avoid deadlock by swapping forks without deinitializing them(breaking symmetry?)
        if i == forks.len() - 1 {
            std::mem::swap(&mut left_fork, &mut right_fork);
        }
        let philosopher = Philosopher {
            name: PHILOSOPHERS[i].to_string(),
            thoughts: tx,
            left_fork,
            right_fork,
        };
        thread::spawn(move || {
            for _ in 0..100 {
                philosopher.eat();
                philosopher.think();
            }
        });
    }
    drop(tx);
    for thought in rx {
        println!("{}", thought);
    }
}
