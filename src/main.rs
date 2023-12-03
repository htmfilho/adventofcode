use adventofcode::utils;

mod days;

fn main() {
    // Trebuchet
    let day1_input = utils::read_input_file("resources/input/day1.txt");
    let total_day1 = days::trebuchet::sum_calibration_values(day1_input);
    println!("Day 1: {total_day1}");

    // Cube Conundrum
    let day2_input = utils::read_input_file("resources/input/day2.txt");
    let total_day2: (usize, usize) = days::cube_conundrum::sum_possible_games(day2_input, 12, 13, 14);
    println!("Day 2: {} and {}", total_day2.0, total_day2.1);

    let day3_input: Vec<String> = utils::read_input_file("resources/input/day3.txt");
    let total_day3 = days::gear_ratios::sum_part_numbers(day3_input);
    println!("Day 3: {total_day3}");
}
