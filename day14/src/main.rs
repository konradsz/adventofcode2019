use std::collections::HashMap;
use std::collections::VecDeque;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

#[derive(Clone, Eq, PartialEq, Hash)]
struct Ingredient {
    count: u64,
    name: String,
}

impl Ingredient {
    fn new(count: u64, name: &str) -> Self {
        Ingredient {
            count,
            name: name.to_string(),
        }
    }
}

fn count_required_ingredients(
    reactions: &HashMap<Ingredient, Vec<Ingredient>>,
    required: Ingredient,
) -> u64 {
    let mut total = 0;
    let mut queue = VecDeque::new();
    queue.push_back(required);

    let mut reserve: HashMap<String, u64> = HashMap::new();

    while !queue.is_empty() {
        let currently_required = queue.pop_front().unwrap();

        if currently_required.name == "ORE" {
            total += currently_required.count;
            continue;
        }

        let mut really_needed = 0;
        let product_in_reserve = reserve.entry(currently_required.name.clone()).or_insert(0);
        if *product_in_reserve >= currently_required.count {
            *product_in_reserve -= currently_required.count;
        } else {
            really_needed = currently_required.count - *product_in_reserve;
            *product_in_reserve = 0;
        }

        for (product, substrates) in reactions.iter() {
            if product.name == currently_required.name {
                let number_of_reactions =
                    (really_needed as f64 / product.count as f64).ceil() as u64;
                for substrate in substrates.iter() {
                    let produced = number_of_reactions * substrate.count;
                    queue.push_back(Ingredient::new(produced, &substrate.name));
                }
                *reserve.entry(currently_required.name.clone()).or_insert(0) +=
                    product.count * number_of_reactions - really_needed;
            }
        }
    }

    total
}

fn part_1(reactions: &HashMap<Ingredient, Vec<Ingredient>>) {
    let total = count_required_ingredients(reactions, Ingredient::new(1, "FUEL"));
    assert_eq!(total, 483_766);
}

fn part_2(reactions: &HashMap<Ingredient, Vec<Ingredient>>) {
    let mut low = 1;
    let mut high = 10_000_000; // random number that gives more than 1_000_000_000_000 ORE

    let mut middle = 0;
    while high - low != 1 {
        middle = (low + high) / 2;
        let total = count_required_ingredients(&reactions, Ingredient::new(middle, "FUEL"));

        if total > 1_000_000_000_000 {
            high = middle;
        } else {
            low = middle;
        }
    }

    assert_eq!(middle, 3_061_522);
}

fn main() -> io::Result<()> {
    let file = File::open("input")?;
    let reader = BufReader::new(file);

    let parse_ingredient = |description: &str| -> Ingredient {
        let mut split = description.split_whitespace();
        Ingredient::new(
            split.next().unwrap().parse::<u64>().unwrap(),
            split.next().unwrap(),
        )
    };

    let mut reactions = HashMap::new();

    for line in reader.lines() {
        let line = line.unwrap();
        let mut split = line.split(" => ");

        let mut substrates = Vec::new();
        let substrates_str = split.next().unwrap();
        for substrate_str in substrates_str.split(',') {
            substrates.push(parse_ingredient(substrate_str));
        }
        let product_str = split.next().unwrap();
        let product = parse_ingredient(product_str);

        reactions.insert(product, substrates);
    }

    part_1(&reactions);
    part_2(&reactions);

    Ok(())
}
