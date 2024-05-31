mod prints;

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_items_from_file(filename: &str) -> Result<HashMap<String, i32>, std::io::Error> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let mut item_map = HashMap::new();

    for line in reader.lines() {
        if let Ok(line) = line {
            if let Some((item, quantity_str)) = line.split_once(": ") {
                if let Ok(quantity) = quantity_str.parse::<i32>() {
                    item_map.insert(item.to_string(), quantity);
                }
            }
        }
    }

    Ok(item_map)
}

fn main() {
    println!("{}", prints::print_nuka_cola_machine());
    println!("{}", prints::print_intro());

    // read nuka cola prices into a vector from a txt file
    // should really use json or something though
    let filename = "prices.txt";
    match read_items_from_file(filename) {
        Ok(item_map) => {
            /*
            println!("prices.txt file printed next:");
            for (item, quantity) in item_map {
                println!("{}: {}", item, quantity);
            }
            */
        }
        Err(err) => {
            eprintln!("Error reading file: {}", err);
        }
    }
}
