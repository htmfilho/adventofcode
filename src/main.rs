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
    use std::collections::HashMap;

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

    fn calibration_digits(line: String) -> (i32, i32) {
        if line.is_empty() {
            return (-1, -1)
        }

        let mut pos_first: i32 = -1;
        let mut str_first: String = "".to_string();
        let mut pos_last: i32 = -1;
        let mut str_last: String = "".to_string();

        let digits = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "1", "2", "3", "4", "5", "6", "7", "8", "9"];
        let digits_number = HashMap::from([
            (digits[0], 1), (digits[1], 2),  (digits[2], 3),  (digits[3], 4), (digits[4], 5), (digits[5], 6), (digits[6], 7), (digits[7], 8), (digits[8], 9),
            (digits[9], 1), (digits[10], 2), (digits[11], 3), (digits[12], 4), (digits[13], 5), (digits[14], 6), (digits[15], 7), (digits[16], 8), (digits[17], 9)
        ]);

        for str_digit in digits {
            let pos_f = match line.find(str_digit) {
                Some(p) => p as i32,
                None => -1
            };

            let pos_l = match line.rfind(str_digit) {
                Some(p) => p as i32,
                None => -1
            };

            if pos_first < 0 || pos_last < 0 {
                pos_first = pos_f;
                str_first = str_digit.to_string();
                pos_last = pos_l;
                str_last = str_digit.to_string();
                continue;
            }

            if pos_f >= 0 && pos_f < pos_first {
                pos_first = pos_f;
                str_first = str_digit.to_string();
            } 
            
            if pos_l >= 0 && pos_l > pos_last {
                pos_last = pos_l;
                str_last = str_digit.to_string();
            }
        }

        let first = match digits_number.get(str_first.as_str()) {
            Some(f) =>  f,
            None => &-1
        };

        let last = match digits_number.get(str_last.as_str()) {
            Some(l) =>  l,
            None => &-1
        };
        
        return (*first, *last);
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
