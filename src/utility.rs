use std::collections::HashMap;
use std::io::{BufRead, BufReader};
use std::fs::File;

use rand::seq::SliceRandom;
use rand::thread_rng;

pub fn read_items_from_file(filename: &str) -> Result<HashMap<String, i32>, std::io::Error> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let mut drink_map = HashMap::new();

    for line in reader.lines() {
        if let Ok(line) = line {
            if let Some((item, quantity_str)) = line.split_once(": ") {
                if let Ok(quantity) = quantity_str.parse::<i32>() {
                    drink_map.insert(item.to_string(), quantity);
                }
            }
        }
    }

    Ok(drink_map)
}

pub fn hashmap_to_vector(mut drink_map: HashMap<String, i32>) -> Vec<(String, i32)> {
    let mut rng = thread_rng();
    let selected_drinks: Vec<(String, i32)> = drink_map
        .drain()
        .collect();

    //randomly select 3 drinks
    let mut shuffled_drinks = selected_drinks.clone();
    shuffled_drinks.shuffle(&mut rng);

    shuffled_drinks.into_iter().take(3).collect()
}

