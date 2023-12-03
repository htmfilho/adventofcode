use adventofcode::utils;

mod days;

fn main() {
    // Trebuchet
    let day1_input = utils::read_input_file("resources/input/day1.txt");
    let total_day1 = days::trebuchet::sum_calibration_values(day1_input);
    println!("Trebuchet Total: {total_day1}");

    // Cube Conundrum
    let day2_input = utils::read_input_file("resources/input/day2.txt");
    let total_day2: (usize, usize) = days::cube_conundrum::sum_possible_games(day2_input, 12, 13, 14);
    print!("Cube Conundrum Total: {}\n  Power: {}", total_day2.0, total_day2.1);
}
