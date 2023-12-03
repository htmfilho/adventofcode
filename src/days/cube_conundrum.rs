struct Game {
    id          : usize,
    subsets     : Vec<Vec<usize>>,
    minimum_set : Vec<usize>,
}

/// Problem Statement:
/// 
/// As you walk, the Elf shows you a small bag and some cubes which are either 
/// red, green, or blue. Each time you play this game, he will hide a secret 
/// number of cubes of each color in the bag, and your goal is to figure out 
/// information about the number of cubes.
/// 
/// To get information, once a bag has been loaded with cubes, the Elf will 
/// reach into the bag, grab a handful of random cubes, show them to you, and 
/// then put them back in the bag. He'll do this a few times per game.
/// 
/// You play several games and record the information from each game (your 
/// puzzle input). Each game is listed with its ID number (like the 11 in Game 11: ...) 
/// followed by a semicolon-separated list of subsets of cubes that were revealed 
/// from the bag (like 3 red, 5 green, 4 blue).
/// 
/// For example, the record of a few games might look like this:
/// 
/// Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
/// Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
/// Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
/// Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
/// Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
/// 
/// In game 1, three sets of cubes are revealed from the bag (and then put back again). 
/// The first set is 3 blue cubes and 4 red cubes; the second set is 1 red cube, 
/// 2 green cubes, and 6 blue cubes; the third set is only 2 green cubes.
/// 
/// The Elf would first like to know which games would have been possible if the bag 
/// contained only 12 red cubes, 13 green cubes, and 14 blue cubes?
/// 
/// In the example above, games 1, 2, and 5 would have been possible if the bag 
/// had been loaded with that configuration. However, game 3 would have been 
/// impossible because at one point the Elf showed you 20 red cubes at once; 
/// similarly, game 4 would also have been impossible because the Elf showed you 
/// 15 blue cubes at once. If you add up the IDs of the games that would have 
/// been possible, you get 8.
/// 
/// Determine which games would have been possible if the bag had been loaded 
/// with only 12 red cubes, 13 green cubes, and 14 blue cubes. What is the sum
/// of the IDs of those games?
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