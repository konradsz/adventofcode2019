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

fn count_required_base_ingredients(
    reactions: &HashMap<Ingredient, Vec<Ingredient>>,
    required_ingredient: &Ingredient,
    needed_base: &mut HashMap<String, u32>,
) {
    println!("count for: {} {}", required_ingredient.name, required_ingredient.count);
    for (product, substrates) in reactions.iter() {
        if product.name == required_ingredient.name {
            // if let Some(needed_base.entry()) count++ else return;
            for (base_name, count) in needed_base.iter_mut() {
                if *base_name == required_ingredient.name {
                    println!("require {} {}", required_ingredient.name, required_ingredient.count);
                    *count += required_ingredient.count;
                    return;
                }
            }

            substrates.iter().for_each(|substrate| {
                count_required_base_ingredients(
                    reactions,
                    &Ingredient::new((required_ingredient.count as f64 / product.count as f64).ceil() as u32 * substrate.count, &substrate.name),
                    needed_base,
                )
            });
        }
    }
}

fn main() -> io::Result<()> {
    let file = File::open("input")?;
    let reader = BufReader::new(file);

    let parse_ingredient = |description: &str| -> Ingredient {
        let mut split = description.split_whitespace();
        Ingredient::new(split.next().unwrap().parse::<u32>().unwrap(), split.next().unwrap())
    };


    let mut base_reactions = HashMap::new();
    let mut reactions = HashMap::new();
    let mut needed_base = HashMap::new();

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

        if substrates.len() == 1 && substrates[0].name == "ORE" {
            base_reactions.insert(product.clone(), substrates);
            needed_base.insert(product.name.clone(), 0);
        } else {
            reactions.insert(product, substrates);
        }
    }
    reactions.extend(base_reactions.clone());

    let required = Ingredient::new(1, "FUEL");

    count_required_base_ingredients(&reactions, &required, &mut needed_base);

    let mut total = 0;
    for (name, needed_count) in needed_base.iter() {
        //println!("{}: {}", name, count);
        for (product, substrate) in base_reactions.iter() {
            if *name == product.name {
                println!(
                    "needed: {}, product.count: {}, substrate.count: {}",
                    needed_count, product.count, substrate[0].count
                );
                total += (*needed_count as f64 / product.count as f64).ceil() as u32
                    * substrate[0].count;
            }
        }
    }

    println!("{}", total);

    Ok(())
}
