
use dec1::DepthScanner;
use std::env;
use a2021::process_lines;

pub mod dec1 {
    use a2021::WindowOperator;

    pub struct DepthScanner {
        prev : u32,
        count : i32,
        number_seen : usize,
        window_size : usize,
        window_operator : WindowOperator,
    }

    impl DepthScanner {
        pub fn new(window_size : usize) -> DepthScanner {
            DepthScanner{
                prev : u32::MAX,
                count : 0,
                number_seen : 0,
                window_size,
                window_operator : WindowOperator::new(window_size),
            }
        }

        pub fn process_line(&mut self, line : &String) {
            let n : u32 = line.parse().unwrap();
            let sum = self.window_operator.add_and_evaluate(n);
            self.number_seen = self.number_seen + 1;
            if self.number_seen > self.window_size && self.prev < sum {
                self.count = self.count + 1;
            }
            println!("n {} prev {} sum {} ns {} count {}", n, self.prev, sum ,self.number_seen, self.count);
            self.prev = sum;
        }

        pub fn final_count(&self) -> i32 {
            return self.count;
        }
    }

} // end mod dec1

#[test]
fn test_depthscanner() {
    let mut depth_scanner = DepthScanner::new(1);
    const TEST_INPUT: &'static [&'static str] = &["199", "200", "208", "210", "200",  "207", "240", "269", "260", "263"];
    TEST_INPUT.iter().for_each(| s : &&str| depth_scanner.process_line(&s.to_string()));
    assert_eq!(7, depth_scanner.final_count());

    depth_scanner = DepthScanner::new(3);
    TEST_INPUT.iter().for_each(| s : &&str| depth_scanner.process_line(&s.to_string()));
    assert_eq!(5, depth_scanner.final_count());
}


fn main() {
    // wrong answer for window size 3: 1754, 1755
    let args: Vec<String> = env::args().collect();
    let look_behind : usize = args[1].parse().unwrap();
    println!("{}", look_behind);

    let mut depth_scanner = DepthScanner::new(look_behind);
    process_lines(&args[2], |line : &String| depth_scanner.process_line(line));
    println!("{}", depth_scanner.final_count());
}