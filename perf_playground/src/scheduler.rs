use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread::{self, sleep};
use std::time::{Duration, Instant};

// Define the Message enum
enum Task {
    Add(i32, i32),
    Subtract(i32, i32),
    Multiply(i32, i32),
    Divide(i32, i32),
}

// Implement the Scheduler
struct Scheduler {
    sender: Sender<Task>,
}

impl Scheduler {
    fn new() -> (Self, Receiver<Task>) {
        let (sender, receiver) = channel();
        (Scheduler { sender }, receiver)
    }

    fn schedule(&self, task: Task) {
        self.sender.send(task).unwrap();
    }
}

pub fn schedule() {
    let (scheduler, receiver) = Scheduler::new();

    // Spawn a thread to handle tasks
    thread::spawn(move || {
        while let Ok(task) = receiver.recv() {
            match task {
                Task::Add(a, b) => {
                    sleep(Duration::from_secs(1));
                    println!("Result: {}", a + b);
                }
                Task::Subtract(a, b) => println!("Result: {}", a - b),
                Task::Multiply(a, b) => println!("Result: {}", a * b),
                Task::Divide(a, b) => {
                    if b != 0 {
                        println!("Result: {}", a / b);
                    } else {
                        println!("Cannot divide by zero");
                    }
                }
            }
        }
    });

    // Schedule some tasks
    scheduler.schedule(Task::Add(1, 5));
    scheduler.schedule(Task::Subtract(1, 5));
    scheduler.schedule(Task::Multiply(1, 5));
    scheduler.schedule(Task::Divide(1, 5));

    // Give some time for tasks to be processed
    thread::sleep(Duration::from_secs(2));
}

use crossbeam::deque::{Injector, Steal};

pub fn injector() {
    let q = Injector::new();
    q.push(1);
    q.push(2);

    assert_eq!(q.steal(), Steal::Success(1));
    assert_eq!(q.steal(), Steal::Success(2));
    assert_eq!(q.steal(), Steal::Empty);
}
