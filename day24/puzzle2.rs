use std::collections::HashMap;

#[derive(Clone, PartialEq)]
enum Tile {
    Bug,
    Empty,
    Subgrid,
}

const SIZE: usize = 5;

fn count_adjacent_bugs(
    area: &Vec<Vec<Tile>>,
    area_around: &Vec<Vec<Tile>>,
    area_inner: &Vec<Vec<Tile>>,
    x: usize,
    y: usize,
) -> u32 {
    let count = |area: &Vec<Vec<Tile>>, x: usize, y: usize| {
        if area[y][x] == Tile::Bug {
            1
        } else {
            0
        }
    };

    let left = || -> u32 {
        if x == 0 {
            count(area_around, 1, 2)
        } else if x == 3 && y == 2 {
        	(0..SIZE).map(|i| count(area_inner, 4, i)).sum()
        } else {
            count(area, x - 1, y)
        }
    };

    let right = || -> u32 {
        if x == SIZE - 1 {
            count(area_around, 3, 2)
        } else if x == 1 && y == 2 {
            (0..SIZE).map(|i| count(area_inner, 0, i)).sum()
        } else {
            count(area, x + 1, y)
        }
    };

    let up = || -> u32 {
        if y == 0 {
            count(area_around, 2, 1)
        } else if x == 2 && y == 3 {
        	(0..SIZE).map(|i| count(area_inner, i, 4)).sum()
        } else {
            count(area, x, y - 1)
        }
    };

    let down = || -> u32 {
        if y == SIZE - 1 {
            count(area_around, 2, 3)
        } else if x == 2 && y == 1 {
        	(0..SIZE).map(|i| count(area_inner, i, 0)).sum()
        } else {
            count(area, x, y + 1)
        }
    };

    left() + right() + up() + down()
}

fn create_empty_area() -> Vec<Vec<Tile>> {
	let mut empty_area = vec![vec![Tile::Empty; SIZE]; SIZE];
    empty_area[2][2] = Tile::Subgrid;
    empty_area
}

fn process(recursive_areas: &mut HashMap<i32, Vec<Vec<Tile>>>, level: i32) {
    if !recursive_areas.contains_key(&level) {
        return;
    }

    let area = recursive_areas.get(&level).unwrap().clone();
    let mut result = create_empty_area();

    let mut area_around = create_empty_area();
    if recursive_areas.contains_key(&(level + 1)) {
        area_around = recursive_areas.get(&(level + 1)).unwrap().clone();
    }

    let mut area_inner = create_empty_area();
    if recursive_areas.contains_key(&(level - 1)) {
        area_inner = recursive_areas.get(&(level - 1)).unwrap().clone();
    }

    for (y, row) in area.iter().enumerate() {
        for (x, tile) in row.iter().enumerate() {
            let adjacent_bugs = count_adjacent_bugs(&area, &area_around, &area_inner, x, y);
            match tile {
                Tile::Bug => {
                    if adjacent_bugs == 1 {
                        result[y][x] = Tile::Bug;
                    }
                }
                Tile::Empty => {
                    if adjacent_bugs == 1 || adjacent_bugs == 2 {
                        result[y][x] = Tile::Bug;
                    }
                }
                Tile::Subgrid => {
                    process(recursive_areas, level - 1);
                }
            }
        }
    }

    let area = recursive_areas.get_mut(&level).unwrap();
    *area = result;
}

fn main() {
    let area = vec![
        vec![Tile::Empty, Tile::Empty, Tile::Bug, Tile::Bug, Tile::Bug],
        vec![Tile::Empty, Tile::Bug, Tile::Bug, Tile::Bug, Tile::Bug],
        vec![Tile::Empty, Tile::Empty, Tile::Subgrid, Tile::Bug, Tile::Empty],
        vec![Tile::Empty, Tile::Bug, Tile::Empty, Tile::Empty, Tile::Bug],
        vec![Tile::Bug, Tile::Empty, Tile::Bug, Tile::Bug, Tile::Bug],
    ];

    let mut recursive_areas = HashMap::new();
    recursive_areas.insert(0, area);

    const MINUTES: i32 = 200;
    let mut added_level = 1;
    for iteration in 1..=MINUTES {
        recursive_areas.insert(-added_level, create_empty_area());
        recursive_areas.insert(added_level, create_empty_area());
        added_level += 1;

        process(&mut recursive_areas, iteration);
    }

    let mut sum = 0;
    for area in recursive_areas.values() {
        for row in area.iter() {
            sum += row.iter().filter(|v| **v == Tile::Bug).count();
        }
    }

    println!("Number of bugs after 200 minutes: {}", sum);
}
