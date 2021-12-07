use std::env;
use a2021::process_lines;
use a2021::WindowOperator;


pub struct DepthScanner {
    prev : u32,
    count : u32,
    window_operator : WindowOperator,
}

impl DepthScanner {
    pub fn new(window_size : usize) -> DepthScanner {
        DepthScanner{
            prev : u32::MAX,
            count : 0,
            window_operator : WindowOperator::new(window_size),
        }
    }

    pub fn process_line(&mut self, line : &String) {
        let n : u32 = line.parse().unwrap();
        let sum = self.window_operator.add_and_evaluate(n);

        if self.prev < sum {
            self.count = self.count + 1;
        }
        self.prev = sum;
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