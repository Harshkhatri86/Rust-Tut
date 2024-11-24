use std::{sync::mpsc, thread};

pub fn channel_multithreading() {
    let (tx, rs) = mpsc::channel();
    thread::spawn(move || {
        let val = String::from("Any Value");
        tx.send(val).unwrap();
    });

    let rec = rs.recv();
    match rec {
        Ok(rec) => println!("Got {}", rec),
        Err(err) => println!("OK with erro {}", err),
    }
}
