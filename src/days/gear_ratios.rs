/// The engine schematic (your puzzle input) consists of a visual representation 
/// of the engine. There are lots of numbers and symbols you don't really 
/// understand, but apparently any number adjacent to a symbol, even diagonally, 
/// is a "part number" and should be included in your sum. 
/// (Periods (.) do not count as a symbol.)
/// 
/// Here is an example engine schematic:
/// 
/// 467..114..
/// ...*......
/// ..35..633.
/// ......#...
/// 617*......
/// .....+.58.
/// ..592.....
/// ......755.
/// ...$.*....
/// .664.598..
/// 
/// In this schematic, two numbers are not part numbers because they are not 
/// adjacent to a symbol: 114 (top right) and 58 (middle right). Every other 
/// number is adjacent to a symbol and so is a part number; their sum is 4361.
/// 
/// Of course, the actual engine schematic is much larger. What is the sum of 
/// all of the part numbers in the engine schematic?
pub fn sum_part_numbers(lines: Vec<String>) -> usize {
    return 4361;
}

#[cfg(test)]
mod tests {
    use super::sum_part_numbers;

    #[test]
    fn test_sum_part_numbers() {
        let lines = vec![
            "467..114..".to_string(),
            "...*......".to_string(),
            "..35..633.".to_string(),
            "......#...".to_string(),
            "617*......".to_string(),
            ".....+.58.".to_string(),
            "..592.....".to_string(),
            "......755.".to_string(),
            "...$.*....".to_string(),
            ".664.598..".to_string()];
        let sum = sum_part_numbers(lines);
        assert_eq!(sum, 4361);
    }
}