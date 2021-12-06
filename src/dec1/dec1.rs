use std::env;
use a2021::process_lines;
use ringbuf::RingBuffer;

fn main() {
    let args: Vec<String> = env::args().collect();
    let look_behind : usize = args[1].parse().unwrap();
    println!("{}", look_behind);
    let mut prev = u32::MAX;
    let ring_buffer = RingBuffer::<u32>::new(look_behind);
    let (mut prod, mut cons) = ring_buffer.split();

    let mut count : u32 = 0;
    let f = |line : &String| {
        let n : u32 = line.parse().unwrap();
        if prod.is_full() {
            cons.pop();
        }
        prod.push(n).unwrap();

        let mut window_count : u32 = 0;
        cons.for_each(|depth : &u32| {
            //println!("{}", depth);
            window_count += depth;
        });
        //println!("{}", window_count);
        if prev < window_count {
            count = count + 1;
        }
        prev = window_count;
    };

    process_lines(&args[2], f);
    println!("{}", count)
}