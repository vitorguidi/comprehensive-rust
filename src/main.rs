use std::sync::{Arc, Mutex};
use std::{result, thread};
use std::sync::mpsc::channel;
use std::time::Duration;

fn main() {
    let mut results = Vec::new();
    let loops = 10;
    let const_val = 5;

    for i in 1..loops+1 {
        results.push(thread::spawn(move || -> i32 {
            thread::sleep(Duration::from_millis(5));
            const_val
        }));
    }

    let mut cnt = 0;

    for result in results {
        if let Ok(value) = result.join() {
            cnt += value;
        }
    }

    assert_eq!(const_val * loops, cnt);
    
}