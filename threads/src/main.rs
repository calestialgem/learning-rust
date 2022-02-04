use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx1, rx) = mpsc::channel(); // multiple producer, single consumer
    let tx2 = tx1.clone(); // second producer
                           // first thread
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];
        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    // second thread
    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];
        for val in vals {
            tx2.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    // iteration ends when the channel is closed
    // until then it waits for any data
    for received in rx {
        println!("Got: {}", received);
    }
}
