use std::{collections::HashMap, fs::File, io::Read};

pub fn part_one() {
    const MAX_RED_CUBES: u32 = 12;
    const MAX_GREEN_CUBES: u32 = 13;
    const MAX_BLUE_CUBES: u32 = 14;
    // Read yaml file
    let advent_day2_yaml_path = "src/advent_day2_data.yaml";
    let advent_day2_yaml_content =
        read_yaml_file(advent_day2_yaml_path).expect("Erreur de lecture du fichier YAML");

    let yaml: serde_json::Value = serde_yaml::from_str(&advent_day2_yaml_content).unwrap();
    let serialize_advent_day2 = serde_json::to_string(&yaml).unwrap();
    let advent_day2_json = serde_json::from_str::<HashMap<String, String>>(&serialize_advent_day2)
        .expect("unserialization error");

    let mut amount_of_id_of_game_playable = 0;
    // println!("{:?}", advent_day2_json);
    // iterate trough the games
    for (key, val) in advent_day2_json.iter() {
        // println!("{}", key);
        let mut is_game_playable = true;
        // for each game get the colors of each draw
        for game in val.split("; ") {
            for color_count in game.split(", ") {
                let parts: Vec<&str> = color_count.split(' ').collect();
                // println!("{}", color_count);
                if let Some(count_str) = parts.get(0) {
                    // println!("{}", count_str);
                    if let Ok(count) = count_str.parse::<u32>() {
                        if let Some(color) = parts.get(1) {
                            if color.to_string() == "green" && count > MAX_GREEN_CUBES {
                                is_game_playable = false
                            }
                            if color.to_string() == "red" && count > MAX_RED_CUBES {
                                is_game_playable = false
                            }
                            if color.to_string() == "blue" && count > MAX_BLUE_CUBES {
                                is_game_playable = false
                            }
                            // println!("{}", is_game_playable)
                        }
                    }
                }
            }
        }

        if is_game_playable {
            let id_game_str: Option<&str> = key.split(' ').collect::<Vec<&str>>().get(1).copied();
            if let Some(id_str) = id_game_str {
                if let Ok(id) = id_str.parse::<i32>() {
                    // Utilisation de id (qui est maintenant un entier)
                    println!("ID du jeu : {}", id);
                    println!("{}",amount_of_id_of_game_playable);
                    amount_of_id_of_game_playable += id
                } 
            }

        }
    }
    println!("{}", amount_of_id_of_game_playable);
}

fn read_yaml_file(file_path: &str) -> std::io::Result<String> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}
