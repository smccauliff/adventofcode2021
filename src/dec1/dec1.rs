mod example;

use std::env;
use a2021::process_lines;
use ringbuf::RingBuffer;
use ringbuf::Consumer;
use ringbuf::Producer;


pub struct DepthScanner {
    prev : u32,
    count : u32,
    ring_buffer : RingBuffer<u32>,
    producer : Option<Producer<u32>>,
    consumer : Option<Consumer<u32>>,
}

impl DepthScanner {
    pub fn new(look_behind : usize) -> DepthScanner {
        let mut me = DepthScanner{
            prev : u32::MAX,
            count : 0,
            ring_buffer : RingBuffer::<u32>::new(look_behind),
            producer : None,
            consumer : None,
        };
        me.producer = Some(me.ring_buffer.split().0);
        me.consumer = Some(me.ring_buffer.split().1);

        return me;
    }

    pub fn process_line(&mut self, line : &String) {
        let n : u32 = line.parse().unwrap();
        if self.producer.unwrap().is_full() {
            self.consumer.unwrap().pop();
        }
        self.producer.unwrap().push(n).unwrap();

        let mut window_count : u32 = 0;
        self.consumer.unwrap().for_each(|depth : &u32| {
            //println!("{}", depth);
            window_count += depth;
        });
        if self.prev < window_count {
            self.count = self.count + 1;
        }
        self.prev = window_count;
    }

    pub fn final_count(&self) -> u32 {
        return self.count;
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let look_behind : usize = args[1].parse().unwrap();
    println!("{}", look_behind);

    let mut depth_scanner = DepthScanner::new(look_behind);
    process_lines(&args[2], |line : &String| depth_scanner.process_line(line));
    println!("{}", depth_scanner.final_count());
}