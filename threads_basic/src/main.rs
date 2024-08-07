use std::thread;
use std::sync::mpsc;
use std::time::Duration;
use std::sync::{Mutex,Arc};

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
    ///////////////////////////////////////////////////////////////////
    let v = vec![1, 2, 3];

    let handle1 = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    handle1.join().unwrap();

    ///////////////////////////////////////////////////////////////////
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);
    ///////////////////////////////////////////////////////////////////
    let (tx1, rx1) = mpsc::channel();

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

    for received in rx1 {
        println!("Got: {}", received);
    }

    ///////////////////////////////////////////////////////////////////
    let (tx2, rx2) = mpsc::channel();

    let tx3 = tx2.clone();
    thread::spawn(move || { 
        let vals = vec![ 
        String::from("Hello"),
        String::from("here"),
        String::from("more"),
        String::from("in two"),
    ];

        for val in vals {
            tx3.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || { 
        let vals = vec![ 
        String::from("you"),
        String::from("have"),
        String::from("messages"),
        String::from("threads"),
    ];

        for val in vals {
            tx2.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx2 {
        println!("Got: {}", received);
    }

    ///////////////////////////////////////////////////////////////////
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m = {:?}", m);

    ///////////////////////////////////////////////////////////////////
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
