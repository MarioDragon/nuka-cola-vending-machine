mod prints;
mod utility;

fn main() {
    println!("{}", prints::print_nuka_cola_machine());
    println!("{}", prints::print_intro());

    // read nuka cola prices into a vector from a txt file
    // should really use json or something though
    let filename = "prices.txt";
    match utility::read_items_from_file(filename) {
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
