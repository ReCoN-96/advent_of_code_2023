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

    let mut number_of_game_playable = 0;
    // iterate trough the games
    for (_key, val) in advent_day2_json.iter() {
        let mut color_counts: HashMap<&str, u32> = HashMap::new();
        // for each game get the colors of each draw
        for game in val.split("; ") {
            for color_count in game.split(", ") {
                let parts: Vec<&str> = color_count.split(' ').collect();
                if let Some(count_str) = parts.get(0) {
                    if let Ok(count) = count_str.parse::<u32>() {
                        if let Some(color) = parts.get(1) {
                            let count_entry = color_counts.entry(color).or_insert(0);
                            *count_entry += count;
                        }
                    }
                }
            }
        }
        // if color_counts.get("red") =< MAX_RED_CUBES && color_counts.get("blue") =< MAX_BLUE_CUBES && color_counts.get("green") =< MAX_GREEN_CUBES {
        //     println!("{:?}", color_counts)
        // }
        if *color_counts.get("red").unwrap_or(&0) <= MAX_RED_CUBES
            && *color_counts.get("blue").unwrap_or(&0) <= MAX_BLUE_CUBES
            && *color_counts.get("green").unwrap_or(&0) <= MAX_GREEN_CUBES
        {
            number_of_game_playable += 1
        }
    }
    println!("{}", number_of_game_playable);
}

fn read_yaml_file(file_path: &str) -> std::io::Result<String> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}
