use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn calculate_fuel(mass: i32) -> i32 {
    (mass / 3) - 2
}

fn main() -> io::Result<()> {
    let file = File::open("input")?;
    let reader = BufReader::new(file);

    let mut sum = 0;
    for line in reader.lines() {
        let mass: i32 = line.unwrap().parse().unwrap();
        sum += calculate_fuel(mass);
    }

    println!("{}", sum);

    Ok(())
}
