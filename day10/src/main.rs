use std::fs::File;
use std::io::{self, prelude::*, BufReader};

#[derive(PartialEq)]
enum Field {
    Empty,
    Asteroid,
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Data {
    offset_x: i32,
    offset_y: i32,
    angle: f64,
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

fn destroy_asteroids(map: &Vec<Vec<Field>>, source_x: usize, source_y: usize) {
    let mut asteroids_data = Vec::new();

    for (y, row) in map.iter().enumerate() {
        for (x, _) in row.iter().enumerate().filter(|(_, f)| **f != Field::Empty) {
            if x == source_x && y == source_y {
                continue;
            }

            let (offset_x, offset_y) = (x as i32 - source_x as i32, y as i32 - source_y as i32);
            let angle = calculate_angle(offset_x, offset_y);
            asteroids_data.push(Data {
                offset_x,
                offset_y,
                angle,
            });
        }
    }

    let mut counter = 1;
    let mut next_pass;
    while asteroids_data.len() != 0 {
        // RIGHT SIDE
        asteroids_data.sort_by(|lhs, rhs| lhs.angle.partial_cmp(&rhs.angle).unwrap());
        next_pass = asteroids_data.clone();
        let mut indices_to_remove = Vec::new();
        loop {
            let mut next_iteration_data: Vec<Data> = Vec::new();
            let right_side_match = asteroids_data
                .iter()
                .filter(|data| data.offset_x >= 0)
                .next();
            if right_side_match.is_none() {
                break;
            }

            let right_side_match = right_side_match.unwrap();
            let mut right_best_match = right_side_match;
            let min_angle = right_best_match.angle;

            let mut distance = std::f64::MAX;
            for entry in asteroids_data.iter().filter(|data| {
                data.offset_x >= 0 && (data.angle - min_angle).abs() < std::f64::EPSILON
            }) {
                let distance_from_entry =
                    f64::from(entry.offset_x.pow(2) + entry.offset_y.pow(2)).sqrt();
                if distance_from_entry < distance {
                    right_best_match = entry;
                    distance = distance_from_entry;
                }
            }

            println!("#{}: destroyed {:?}", counter, right_best_match);
            counter += 1;

            for entry in asteroids_data.iter() {
                if entry != right_best_match && (entry.angle - min_angle).abs() > std::f64::EPSILON
                {
                    next_iteration_data.push(*entry);
                }
            }

            for (index, a) in next_pass.iter().enumerate() {
                if a == right_best_match {
                    indices_to_remove.push(index);
                }
            }

            if next_iteration_data.len() == 0 {
                break;
            }
            asteroids_data = next_iteration_data;
        }

        indices_to_remove.sort();
        for i in indices_to_remove.iter().rev() {
            next_pass.remove(*i);
        }
        indices_to_remove.clear();
        asteroids_data = next_pass.clone();

        // LEFT SIDE
        asteroids_data.sort_by(|lhs, rhs| rhs.angle.partial_cmp(&lhs.angle).unwrap());
        loop {
            let mut next_iteration_data: Vec<Data> = Vec::new();
            let left_side_match = asteroids_data
                .iter()
                .filter(|data| data.offset_x < 0)
                .next();
            if left_side_match.is_none() {
                break;
            }

            let left_side_match = left_side_match.unwrap();
            let mut left_best_match = left_side_match;
            let min_angle = left_best_match.angle;

            let mut distance = std::f64::MAX;
            for entry in asteroids_data.iter().filter(|data| {
                data.offset_x < 0 && (data.angle - min_angle).abs() < std::f64::EPSILON
            }) {
                let distance_from_entry =
                    f64::from(entry.offset_x.pow(2) + entry.offset_y.pow(2)).sqrt();
                if distance_from_entry < distance {
                    left_best_match = entry;
                    distance = distance_from_entry;
                }
            }

            println!("#{}: destroyed {:?}", counter, left_best_match);
            counter += 1;

            for entry in asteroids_data.iter() {
                if entry != left_best_match && (entry.angle - min_angle).abs() > std::f64::EPSILON {
                    next_iteration_data.push(*entry);
                }
            }

            for (index, a) in next_pass.iter().enumerate() {
                if a == left_best_match {
                    indices_to_remove.push(index);
                }
            }

            if next_iteration_data.len() == 0 {
                break;
            }
            asteroids_data = next_iteration_data;
        }
        indices_to_remove.sort();
        for i in indices_to_remove.iter().rev() {
            next_pass.remove(*i);
        }
        indices_to_remove.clear();
        asteroids_data = next_pass.clone();
    }
}

fn part_1() -> io::Result<()> {
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

    assert_eq!(max, 227);
    assert_eq!(best_x, 11);
    assert_eq!(best_y, 13);

    Ok(())
}

fn part_2() -> io::Result<()> {
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

    destroy_asteroids(&map, 11, 13);

    Ok(())
}

fn main() -> io::Result<()> {
    part_1()?;
    part_2()?;

    Ok(())
}

/*
use std::fs::File;
use std::io::{self, prelude::*, BufReader};





fn main() -> io::Result<()> {

}
*/
