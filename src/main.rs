use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();
    let handler_send = thread::spawn(move || {
        let val = 2;
        if let Ok(()) =tx.send(val) {
            println!("send thread succeeded, sent {}", val) 
        } else {
            println!("send thread failed") 
        }
    });
    let handler_rcv = thread::spawn(move || {
        if let Ok(val) = rx.recv() {
            println!("rcv thread succeeded: {}", val) 
        } else {
            println!("rcv thread failed") 
        }
    });

    handler_send.join();
    handler_rcv.join();
}