use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::collections::HashMap;

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
struct Ingredient {
    count: u32,
    name: String,
}

impl Ingredient {
    fn new(count: u32, name: &str) -> Self {
        Ingredient {
            count,
            name: name.to_string(),
        }
    }
}

fn count_required_ingredients(
    reactions: &HashMap<Ingredient, Vec<Ingredient>>,
    required_ingredient: &Ingredient,
    reserve: &mut HashMap<String, u32>,
) -> u32 {
    let mut total = 0;
    println!("reserve: {:?}", reserve);

    let reserve_count = reserve.entry(required_ingredient.name.clone()).or_insert(0);
    if *reserve_count >= required_ingredient.count {
        *reserve_count -= required_ingredient.count;
        return 0;
    }

    let really_needed = required_ingredient.count - *reserve_count;
    *reserve_count = 0;
    println!("to produce: {}({}) {} we need: ", required_ingredient.count, really_needed, required_ingredient.name);

    for (product, substrates) in reactions.iter() {
        if product.name == required_ingredient.name {
            for substrate in substrates.iter() {
                //println!("{} really_needed: {}", required_ingredient.name, really_needed);

                let number_of_reactions = (really_needed as f64 / product.count as f64).ceil() as u32;
                let produced = number_of_reactions * substrate.count;
                //let needed = (really_needed as f64 / product.count as f64).floor() as u32 * substrate.count;
                let excess = number_of_reactions * product.count - really_needed;
                println!("{} {}, excess: {} {}", produced, substrate.name, excess, required_ingredient.name);
                *reserve.entry(required_ingredient.name.clone()).or_insert(0) += excess;


                if substrate.name != "ORE" {
                    // produced -> needed
                    total += count_required_ingredients(reactions, &Ingredient::new(produced , &substrate.name), reserve);
                } else {
                    return produced;
                }
            }
        }
    }

    total
}

fn main() -> io::Result<()> {
    let file = File::open("input")?;
    let reader = BufReader::new(file);

    let parse_ingredient = |description: &str| -> Ingredient {
        let mut split = description.split_whitespace();
        Ingredient::new(split.next().unwrap().parse::<u32>().unwrap(), split.next().unwrap())
    };

    let mut reactions = HashMap::new();

    for line in reader.lines() {
        let line = line.unwrap();
        let mut split = line.split(" => ");

        let mut substrates = Vec::new();
        let substrates_str = split.next().unwrap();
        for substrate_str in substrates_str.split(",") {
            substrates.push(parse_ingredient(substrate_str));
        }
        let product_str = split.next().unwrap();
        let product = parse_ingredient(product_str);

        reactions.insert(product, substrates);
    }

    let required = Ingredient::new(1, "FUEL");
    let mut reserve = HashMap::new();
    let total = count_required_ingredients(&reactions, &required, &mut reserve);

    println!("{}", total);

    Ok(())
}
