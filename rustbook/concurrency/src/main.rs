use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {i} from the spawned thread!");
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {i} from the main thread!");
        thread::sleep(Duration::from_millis(1));
    }
    handle.join().unwrap();

    // moving data to new thread
    let v = vec![1, 2, 3];
    let handle = thread::spawn(move || {
        println!("Here's a vector: {v:?}");
    });
    handle.join().unwrap();

    // ***
    // thread channels & messaging
    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_millis(300));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(300));
        }
    });

    // block main/rx thread until a message is received (use try_recv for non-blocking call)
    // let received = rx.recv().unwrap();
    // println!("Got: {received}");

    // rx is treated like an iterator, but it still must block on each call (calling .recv() behind the scenes)
    for received in rx {
        println!("Got: {received}");
    }

    // ***
    // threads with shared memory (& mutexes)
    let m = Mutex::new(5);
    {
        let mut num = m.lock().unwrap(); // lock() is a blocking call
        *num = 6;
    }
    println!("m = {m:?}");

    // sharing a mutex between threads
    let counter = Arc::new(Mutex::new(0)); // atomic reference counter (Arc)
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap(); // this will still be a blocking call, but across threads I guess

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
