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