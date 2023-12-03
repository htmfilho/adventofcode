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
    pub mod trebuchet {

        use std::collections::HashMap;

        pub fn sum_calibration_values(lines: Vec<String>) -> i32 {
            let mut sum = 0;
            for line in lines {
                let mut calibration = 0;
                
                let digits = calibration_digits(line.clone());
                if digits.0 >= 0 && digits.1 >= 0 {
                    calibration = (digits.0 * 10) + digits.1;
                }
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
    }

    pub mod cube_conundrum {
        struct Game {
            id          : usize,
            subsets     : Vec<Vec<usize>>,
            minimum_set : Vec<usize>,
        }

        pub fn sum_possible_games(lines: Vec<String>, reds: usize, greens: usize, blues: usize) -> (usize, usize) {
            let mut sum = 0;
            let mut sum_power = 0;
            
            for instructions in lines {
                let game = create_game(instructions);

                // Check if the game is possible
                let mut possible = true;
                for subset in game.subsets {
                    if subset[0] > reds || subset[1] > greens || subset[2] > blues {
                        possible = false;
                        break;
                    }
                }

                if possible {
                    sum += game.id;
                }

                let mut power = 1;
                for colors in game.minimum_set {
                    power *= colors;
                }
                sum_power += power;
            }

            return (sum, sum_power);
        }

        fn create_game(instructions: String) -> Game {
            let game_id = get_id(&instructions);
            let game_subsets = get_subsets(&instructions);
            let minimum_set = get_minimum_set(&game_subsets);
            
            let game = Game {
                id: game_id,
                subsets: game_subsets,
                minimum_set: minimum_set,
            };

            return game;
        }

        fn get_id(instructions: &String) -> usize {
            let game_parts: Vec<&str> = instructions.split(":").collect();
            let id_parts: Vec<&str> = game_parts[0].split(" ").collect();
            let id: usize = id_parts[1].parse().unwrap();
            return id;
        }

        fn get_subsets(instructions: &String) -> Vec<Vec<usize>> {
            let game_parts: Vec<&str> = instructions.split(":").collect();
            let subsets_parts: Vec<&str> = game_parts[1].trim().split(";").collect();
            let mut subsets: Vec<Vec<usize>> = Vec::new(); 
            
            //println!("Instructions: {instructions}");
            for subset_part in subsets_parts {
                let subset_part_withdout_comma = subset_part.trim().replace(',', "");
                let cubes_set: Vec<&str> = subset_part_withdout_comma.trim().split(" ").collect();
                let mut cubes: Vec<usize> = vec![0, 0, 0];
                let mut color_pos = 1;

                for _set in &cubes_set {
                    match cubes_set[color_pos] {
                        "red" => { 
                            cubes[0] = cubes_set[color_pos - 1].parse().unwrap();
                            color_pos += 2;
                        },
                        "green" => {
                            cubes[1] = cubes_set[color_pos - 1].parse().unwrap();
                            color_pos += 2;
                        },
                        "blue" => { 
                            cubes[2] = cubes_set[color_pos - 1].parse().unwrap();
                            color_pos += 2;
                        },
                        _ => println!("subset: '{subset_part_withdout_comma}'"),
                    };

                    if color_pos > cubes_set.len() {
                        break;
                    }
                }
                
                subsets.push(cubes);
            }
            return subsets;
        }

        fn get_minimum_set(subsets: &Vec<Vec<usize>>) -> Vec<usize> {
            let mut minimum_set = vec![0,0,0];
            for subset in subsets {
                if minimum_set[0] < subset[0] {
                    minimum_set[0] = subset[0];
                }

                if minimum_set[1] < subset[1] {
                    minimum_set[1] = subset[1];
                }

                if minimum_set[2] < subset[2] {
                    minimum_set[2] = subset[2];
                }
            }
            return minimum_set;
        }
    }
}

fn main() {
    // Trebuchet
    let day1_input = crate::utils::read_input_file("resources/input/day1.txt");
    let total_day1 = crate::days::trebuchet::sum_calibration_values(day1_input);
    println!("Trebuchet Total: {total_day1}");

    // Cube Conundrum
    let day2_input = crate::utils::read_input_file("resources/input/day2.txt");
    let total_day2: (usize, usize) = crate::days::cube_conundrum::sum_possible_games(day2_input, 12, 13, 14);
    print!("Cube Conundrum Total: {}\n  Power: {}", total_day2.0, total_day2.1);
}
