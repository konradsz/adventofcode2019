use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn calculate_fuel(mass: isize) -> isize {
    (mass / 3) - 2
}

fn part_1() -> io::Result<()> {
    let file = File::open("input")?;
    let reader = BufReader::new(file);

    let sum = reader
        .lines()
        .map(|line| line.unwrap().parse().unwrap())
        .map(calculate_fuel)
        .sum::<isize>();

    assert_eq!(sum, 3_560_353);

    Ok(())
}

fn part_2() -> io::Result<()> {
    let file = File::open("input")?;
    let reader = BufReader::new(file);

    let mut sum = 0;
    for line in reader.lines() {
        let mut mass: isize = line.unwrap().parse().unwrap();

        mass = calculate_fuel(mass);

        while mass > 0 {
            sum += mass;
            mass = calculate_fuel(mass);
        }
    }

    assert_eq!(sum, 5_337_642);

    Ok(())
}

fn main() -> io::Result<()> {
    part_1()?;
    part_2()?;

    Ok(())
}
