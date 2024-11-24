use std::{thread, time::Duration};

pub fn multithreading() {
    thread::spawn(|| {
        for i in 1..10 {
            println!("Process in spwan thread {i}");
            thread::sleep(Duration::from_millis(1));
        }
    });
    for i in 1..10 {
        println!("Process in main thread {i}");
        thread::sleep(Duration::from_millis(1));
    }
}
