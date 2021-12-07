use std::env;
use a2021::process_lines;

pub mod dec2 {
    #[derive(Clone)]
    pub struct Location {
        pub depth : i32,
        pub horizontal : i32,
    }

    pub struct MotionTracker {
        current_location : Location,
    }

    impl MotionTracker {
        pub fn new() -> MotionTracker {
            MotionTracker{
                current_location : Location{ depth: 0, horizontal: 0 },
            }
        }

        pub fn move_location(&mut self, s : &String) {
            let parts = s.split(" ").collect::<Vec<&str>>();
            let direction = parts[0];
            let magnitude : i32 = parts[1].parse().unwrap();
            if direction.eq_ignore_ascii_case("forward") {
                self.current_location.horizontal += magnitude;
            } else if direction.eq_ignore_ascii_case("up") {
                self.current_location.depth -= magnitude;
            } else if direction.eq_ignore_ascii_case("down") {
                self.current_location.depth += magnitude;
            } else {
                panic!();
            }
        }

        pub fn current_location(&self) -> Location {
            return self.current_location.clone();
        }
    }

    pub struct MotionTrackerWithAim {
        current_location : Location,
        aim : i32,
    }

     impl MotionTrackerWithAim {
        pub fn new() -> MotionTrackerWithAim {
            MotionTrackerWithAim{
                current_location : Location{ depth: 0, horizontal: 0 },
                aim : 0,
            }
        }

        pub fn move_location(&mut self, s : &String) {
            let parts = s.split(" ").collect::<Vec<&str>>();
            let direction = parts[0];
            let magnitude : i32 = parts[1].parse().unwrap();
            if direction.eq_ignore_ascii_case("forward") {
                self.current_location.horizontal += magnitude;
                self.current_location.depth += magnitude * self.aim;
            } else if direction.eq_ignore_ascii_case("up") {
                self.aim -= magnitude;
            } else if direction.eq_ignore_ascii_case("down") {
                self.aim += magnitude;
            } else {
                panic!();
            }
        }

        pub fn current_location(&self) -> Location {
            return self.current_location.clone();
        }
    }
}


use dec2::MotionTracker;
use dec2::MotionTrackerWithAim;

#[test]
fn test_motion_tracker() {
 let mut motion_tracker = MotionTracker::new();
    motion_tracker.move_location(&"forward 5".to_string());
    motion_tracker.move_location(&"down 5".to_string());
    motion_tracker.move_location(&"forward 8".to_string());
    motion_tracker.move_location(&"up 3".to_string());
    motion_tracker.move_location(&"down 8".to_string());
    motion_tracker.move_location(&"forward 2".to_string());
    let location = motion_tracker.current_location();
    let dist = location.horizontal * location.depth;
    assert_eq!(150, dist);
}

#[test]
fn test_motion_tracker_with_aim() {
 let mut motion_tracker = MotionTrackerWithAim::new();
    motion_tracker.move_location(&"forward 5".to_string());
    motion_tracker.move_location(&"down 5".to_string());
    motion_tracker.move_location(&"forward 8".to_string());
    motion_tracker.move_location(&"up 3".to_string());
    motion_tracker.move_location(&"down 8".to_string());
    motion_tracker.move_location(&"forward 2".to_string());
    let location = motion_tracker.current_location();
    let dist = location.horizontal * location.depth;
    assert_eq!(900, dist);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut motion_tracker = MotionTrackerWithAim::new();
    process_lines(&args[1], |line : &String| motion_tracker.move_location(line));
    let location = motion_tracker.current_location();
    let dist = location.horizontal * location.depth;
    println!("dist {}", dist);
}