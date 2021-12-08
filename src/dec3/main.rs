use std::env;
use std::ops::Index;
use a2021::process_lines;

pub struct PowerConsumptionState {
    reading_counts : Vec<u32>,
    number_of_readings : u32,
}

impl PowerConsumptionState {
    pub fn new() -> PowerConsumptionState {
        PowerConsumptionState {
            reading_counts : Vec::<u32>::new(),
            number_of_readings : 0,
        }
    }

    pub fn add_power_consumption_reading(&mut self, s : &String) {
        let mut index = 0;
        if self.reading_counts.len() == 0 {
            self.reading_counts.resize(s.len(), 0);
        }
        s.chars().for_each(| c : char| {
            if c == '1' {
                self.reading_counts[index] += 1;
            }
            index += 1;
        });
        self.number_of_readings += 1;
    }

    pub fn final_power_consumption(&self) -> u32 {
        let mut gamma : u32 = 0;
        let mut epsilon : u32 = 0;
        (0..self.reading_counts.len()).for_each(| index : usize| {
            if *self.reading_counts.index(index) > (self.number_of_readings / 2) {
                gamma |= 1;
            } else if *self.reading_counts.index(index) < (self.number_of_readings / 2) {
                epsilon |= 1;
            } else {
                panic!();
            }
            gamma = gamma << 1;
            epsilon = epsilon << 1;
        });

        gamma = gamma >> 1;
        epsilon = epsilon >> 1;
        return gamma * epsilon;
    }
}


#[test]
fn test_pwr() {
    let mut pwr = PowerConsumptionState::new();
    const TEST_INPUT: &'static [&'static str] = &["00100", "11110", "10110", "10111", "10101",  "01111", "00111", "11100", "10000", "11001", "00010", "01010"];
    TEST_INPUT.iter().for_each(|s : &&str| pwr.add_power_consumption_reading(&s.to_string()));
    assert_eq!(198, pwr.final_power_consumption());
}

fn main() {
    // correct answer for part 1 is 852500
    let args: Vec<String> = env::args().collect();
    let mut pwr = PowerConsumptionState::new();
    process_lines(&args[1], |s : &String| pwr.add_power_consumption_reading(s));
    println!("{}", pwr.final_power_consumption());
}
