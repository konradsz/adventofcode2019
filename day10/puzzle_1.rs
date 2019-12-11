use std::fs::File;
use std::io::{self, prelude::*, BufReader};

#[derive(PartialEq)]
enum Field {
    Empty,
    Asteroid,
}

fn calculate_angle(offset_x: i32, offset_y: i32) -> f64 {
    let length = ((offset_x.pow(2) + offset_y.pow(2)) as f64).sqrt();

    offset_y as f64 / length
}

fn count_for_asteroid(map: &Vec<Vec<Field>>, source_x: usize, source_y: usize) -> usize {
    let mut left_to_source = Vec::new();
    let mut right_to_source = Vec::new();
    for (y, row) in map.iter().enumerate() {
        for (x, _) in row.iter().enumerate().filter(|(_, f)| **f != Field::Empty) {
        	if x == source_x && y == source_y {
        		continue;
        	}

            let (offset_x, offset_y) = (x as i32 - source_x as i32, y as i32 - source_y as i32);

            let angle = calculate_angle(offset_x, offset_y);

            if offset_x <= 0 {
                left_to_source.push(angle);
            } else {
                right_to_source.push(angle);
            }
        }
    }

    left_to_source.sort_by(|lhs, rhs| lhs.partial_cmp(rhs).unwrap());
    right_to_source.sort_by(|lhs, rhs| lhs.partial_cmp(rhs).unwrap());

    left_to_source.dedup_by(|lhs, rhs| *lhs - *rhs < std::f64::EPSILON);
    right_to_source.dedup_by(|lhs, rhs| *lhs - *rhs < std::f64::EPSILON);

    left_to_source.len() + right_to_source.len()
}

fn main() -> io::Result<()> {
    let file = File::open("input")?;
    let reader = BufReader::new(file);

    let mut map = Vec::new();

    for line in reader.lines() {
        let mut row = Vec::new();
        for c in line.unwrap().chars() {
            match c {
                '.' => row.push(Field::Empty),
                '#' => row.push(Field::Asteroid),
                _ => panic!(),
            }
        }
        map.push(row);
    }

    let mut max = 0;
    let (mut best_x, mut best_y) = (0, 0);
    for (y, row) in map.iter().enumerate() {
        for (x, _) in row.iter().enumerate().filter(|(_, f)| **f != Field::Empty) {
            let current = count_for_asteroid(&map, x, y);
            if current > max {
                max = current;
                best_x = x;
                best_y = y;
            }
        }
    }

    println!("({}, {}): {}", best_x, best_y, max);

    Ok(())
}
