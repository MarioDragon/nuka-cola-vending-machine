mod prints;
mod utility;

fn main() {
    println!("{}", prints::print_nuka_cola_machine());

    // read nuka cola prices into a vector from a txt file
    // should really use json or something
    let filename = "prices.txt";
    match utility::read_items_from_file(filename) {
        Ok(drink_map) => {

            let randomly_selected_drinks: Vec<(String, i32)> = utility::hashmap_to_vector(drink_map);

            /*
            println!("Randomly selected items:");
            for (drink, quantity) in randomly_selected_drinks {
                println!("{}: {}", drink, quantity);
            }
             */

            println!("{}", prints::print_intro(randomly_selected_drinks));
            
        }
        Err(err) => {
            eprintln!("Error reading file: {}", err);
        }
    }
}