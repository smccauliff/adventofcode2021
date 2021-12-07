use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use ringbuf::RingBuffer;
use ringbuf::Producer;
use ringbuf::Consumer;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

// TODO: handle errors
pub fn process_lines<P, F>(filename : P, mut f : F) -> () where P: AsRef<Path>, F: FnMut(&String) {
    let lines = read_lines(filename).unwrap();
    // Consumes the iterator, returns an (Optional) String
    for line_result in lines {
        f(&line_result.unwrap());
    }
}

pub struct WindowOperator {
    producer : Producer<u32>,
    consumer : Consumer<u32>,
}

impl WindowOperator {
    pub fn new(window_size : usize) -> WindowOperator {
        let (p, c) = RingBuffer::<u32>::new(window_size).split();
        WindowOperator {
            producer : p,
            consumer : c,
        }
    }

    pub fn add_and_evaluate(&mut self, item : u32) -> u32 {
        if self.producer.is_full() {
            self.consumer.pop();
        }
        self.producer.push(item).unwrap();
        let mut window_count = 0;
        self.consumer.for_each(|queued_item : &u32| {
            window_count += queued_item;
        });
        return window_count;
    }
}