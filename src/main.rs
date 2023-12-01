pub mod utils {
    use std::fs;

    pub fn read_input_file(path: &str) -> Vec<String> {
        fs::read_to_string(path)
            .unwrap()
            .split('\n')
            .into_iter()
            .map(String::from)
            .collect()
    }
}

pub mod days {
    pub fn sum_calibration_values(lines: Vec<String>) -> i32 {
        let mut sum = 0;
        for line in lines {
            let mut calibration = 0;
            
            let digits = calibration_digits(line.clone());
            if digits.0 >= 0 && digits.1 >= 0 {
                calibration = (digits.0 * 10) + digits.1;
            }
            println!("Line: '{line}': {calibration} ({sum})");
            sum += calibration;
        }
        return sum;
    }

    pub fn calibration_digits(line: String) -> (i32, i32) {
        let mut first: i32 = -1;
        let mut last: i32 = -1;

        for c in line.chars() {
            if c.is_numeric() && first < 0 {
                first = char_to_digit(c);
                last = char_to_digit(c);
                continue;
            } else if c.is_numeric() {
                last = char_to_digit(c);
            }
        }

        return (first, last);
    }

    fn char_to_digit(c: char) -> i32 {
        let digit = c.to_digit(10);
        return match digit {
            Some(t) => t as i32,
            None => -1
        }
    }
}

fn main() {
    let lines = crate::utils::read_input_file("resources/input/day1.txt");

    let total = crate::days::sum_calibration_values(lines);
    println!("Total: {total}");
}
