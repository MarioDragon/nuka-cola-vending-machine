use std::collections::HashMap;
use std::io::{BufRead, BufReader, self, Write};
use std::fs::File;

use rand::seq::SliceRandom;
use rand::thread_rng;

//maximum number of menu options to choose from
const MENU_MAX: i32 = 3;

pub fn read_items_from_file(filename: &str) -> Result<HashMap<String, i32>, std::io::Error> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let mut drink_map = HashMap::new();

    for line in reader.lines() {
        if let Ok(line) = line {
            if let Some((item, price_str)) = line.split_once(": ") {
                if let Ok(price) = price_str.parse::<i32>() {
                    drink_map.insert(item.to_string(), price);
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

    // randomly select 3 drinks
    let mut shuffled_drinks = selected_drinks.clone();
    shuffled_drinks.shuffle(&mut rng);

    shuffled_drinks.into_iter().take(3).collect()
}

pub fn handle_menu_select() -> i32 {
    let mut input = String::new();

    loop {
        input.clear();

        print!("    Select a menu option: ");
        io::stdout().flush().unwrap();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let input = input.trim();

        // make sure input is a number for menu option
        match input.parse::<i32>() {
            Ok(number) if number > 0 && number < MENU_MAX => {
                println!("    You selected menu option {}.", number);
                return number;
            }
            _ => {
                println!("    Please select a menu option between 1 and {}", MENU_MAX);
            }
        }
    }
}

