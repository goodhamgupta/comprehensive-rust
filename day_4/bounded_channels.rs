// sync channel makes `send` block the thread
// also contains a buffer size, which shows the max number of messages that can be stored on
// the channel
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::sync_channel(3);

    thread::spawn(move || {
        let thread_id = thread::current().id();
        for i in 1..10 {
            tx.send(format!("Message {i}")).unwrap();
            println!("{thread_id:?}: sent message {i}");
        }
        println!("{thread_id:?}: done");
    });

    thread::sleep(Duration::from_millis(100));
    for msg in rx.iter() {
        println!("Main: got {}", msg);
    }
}
