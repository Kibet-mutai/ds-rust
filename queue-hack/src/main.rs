// Enter your code here
use std::env;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::Write;
use std::str::FromStr;

#[derive(Debug)]
struct TwoStackQueue {
    stack_enqueue: Vec<i32>,
    stack_dequeue: Vec<i32>,
}

impl TwoStackQueue {
    fn new() -> Self {
        TwoStackQueue {
            stack_enqueue: Vec::new(),
            stack_dequeue: Vec::new(),
        }
    }

    fn enqueue(&mut self, value: i32) {
        self.stack_enqueue.push(value);
    }

    fn dequeue(&mut self) {
        if self.stack_dequeue.is_empty() {
            while let Some(value) = self.stack_enqueue.pop() {
                self.stack_dequeue.push(value);
            }
        }

        self.stack_dequeue.pop();
    }

    fn print_front(&mut self) -> Option<i32> {
        if !self.stack_dequeue.is_empty() {
            Some(*self.stack_dequeue.last().unwrap())
        } else {
            while let Some(value) = self.stack_enqueue.pop() {
                self.stack_dequeue.push(value);
            }

            self.stack_dequeue.last().copied()
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();
    let number: usize = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse()
        .unwrap();
    let mut queue = TwoStackQueue::new();

    for _ in 0..number {
        let text = stdin_iterator.next().unwrap().unwrap();
        let inputs: Vec<&str> = text.split_whitespace().collect();

        match inputs[0] {
            "1" => {
                let value: i32 = i32::from_str(inputs[1]).unwrap();
                queue.enqueue(value);
            }
            "2" => queue.dequeue(),
            "3" => {
                if let Some(front) = queue.print_front() {
                    writeln!(&mut fptr, "{:?}", front).ok();
                } else {
                    writeln!(&mut fptr, "Queue is empty").ok();
                }
            }
            _ => {
                eprintln!("Invalid query type");
            }
        }
    }
}
