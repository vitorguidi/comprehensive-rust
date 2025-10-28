use std::sync::mpsc::Receiver;
use std::sync::{Arc, Mutex, mpsc};
use std::thread;
use std::time::Duration;

struct Chopstick;

struct Philosopher {
    name: String,
    thoughts: mpsc::Sender<String>,
    idx: usize,
    chopsticks: Arc<Vec<Mutex<bool>>>,
}

impl Philosopher {
    fn think(&self) {
        self.thoughts
            .send(format!("Eureka! {} has a new idea!", &self.name))
            .unwrap();
    }

    fn eat(&self) {
        // Pick up chopsticks...
        let mut idx_left: usize = self.idx;
        let mut idx_right: usize = (self.idx + 1)%PHILOSOPHERS.len();
        if self.idx == PHILOSOPHERS.len() - 1 {
            std::mem::swap(&mut idx_left, &mut idx_right);
        }
        // println!("{} is eating...", &self.name);
        // println!("{} waiting on chopstick {}", self.name, idx_left);
        let _left_chopstick = self.chopsticks[idx_left].lock();
        // println!("{} grabbed chopstick {}", self.name, idx_left);
        // println!("{} waiting on chopstick {}", self.name, idx_right);
        let _right_chopstick = self.chopsticks[idx_right].lock();
        //println!("{} grabbed chopstick {}", self.name, idx_right);
        thread::sleep(Duration::from_millis(10));
        // println!("{} dropped chopstick {}", self.name, idx_left);
        // println!("{} dropped chopstick {}", self.name, idx_right);
    }
}

static PHILOSOPHERS: &[&str] =
    &["Socrates", "Hypatia", "Plato", "Aristotle", "Pythagoras"];

fn main() {
    // Create chopsticks
    let mut chopsticks_locks: Vec<Mutex<bool>> = Vec::new();
    let (tx, rx) = mpsc::channel();
    let mut philosophers: Vec<Philosopher> = Vec::new();
    for i in 0..PHILOSOPHERS.len() {
        chopsticks_locks.push(Mutex::new(false));
    }
    let chopsticks = Arc::new(chopsticks_locks);
    // Create philosophers
    for i in 0..PHILOSOPHERS.len() {
        philosophers.push(Philosopher { 
            name: PHILOSOPHERS[i].to_string(),
            thoughts: tx.clone(),
            idx: i,
            chopsticks: Arc::clone(&chopsticks),
        });
    }
    // Make each of them think and eat 100 times
    drop(tx);

    let mut handlers = Vec::new();

    for phil in philosophers {
        handlers.push(thread::spawn(move ||{
            for _ in 0..100 {
                phil.eat();
                phil.think();
            }
            println!("{} done", &phil.name)
        }));
    }

    for handler in handlers {
        _ = handler.join();
    }
    // Output their thoughts
    for thought in rx.iter() {
        println!("{}", thought);
    }
}