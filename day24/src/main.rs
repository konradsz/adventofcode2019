use std::collections::{HashMap, HashSet};

#[derive(Clone, PartialEq)]
enum Tile {
    Bug,
    Empty,
    Subgrid,
}

const SIZE: usize = 5;

fn count_adjacent_bugs_part_1(area: &[Vec<Tile>], x: usize, y: usize) -> u32 {
    let count = |x: usize, y: usize| {
        if area[y][x] == Tile::Bug {
            1
        } else {
            0
        }
    };

    let left = || -> u32 {
        if x == 0 {
            0
        } else {
            count(x - 1, y)
        }
    };

    let right = || -> u32 {
        if x == SIZE - 1 {
            0
        } else {
            count(x + 1, y)
        }
    };

    let up = || -> u32 {
        if y == 0 {
            0
        } else {
            count(x, y - 1)
        }
    };

    let down = || -> u32 {
        if y == SIZE - 1 {
            0
        } else {
            count(x, y + 1)
        }
    };

    left() + right() + up() + down()
}

fn count_adjacent_bugs_part_2(
    area: &[Vec<Tile>],
    area_around: &[Vec<Tile>],
    area_inner: &[Vec<Tile>],
    x: usize,
    y: usize,
) -> u32 {
    let count = |area: &[Vec<Tile>], x: usize, y: usize| {
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

fn process_part_1(area: &[Vec<Tile>]) -> Vec<Vec<Tile>> {
    let mut result = vec![vec![Tile::Empty; SIZE]; SIZE];

    for (y, row) in area.iter().enumerate() {
        for (x, tile) in row.iter().enumerate() {
            let adjacent_bugs = count_adjacent_bugs_part_1(&area, x, y);
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
                Tile::Subgrid => (),
            }
        }
    }

    result
}

fn process_part_2(recursive_areas: &mut HashMap<i32, Vec<Vec<Tile>>>, level: i32) {
    if !recursive_areas.contains_key(&level) {
        return;
    }

    let area = recursive_areas.get(&level).unwrap().clone();
    let mut result = create_empty_area();

    let area_around = if recursive_areas.contains_key(&(level + 1)) {
        recursive_areas.get(&(level + 1)).unwrap().clone()
    } else {
        create_empty_area()
    };

    let area_inner = if recursive_areas.contains_key(&(level - 1)) {
        recursive_areas.get(&(level - 1)).unwrap().clone()
    } else {
        create_empty_area()
    };

    for (y, row) in area.iter().enumerate() {
        for (x, tile) in row.iter().enumerate() {
            let adjacent_bugs = count_adjacent_bugs_part_2(&area, &area_around, &area_inner, x, y);
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
                    process_part_2(recursive_areas, level - 1);
                }
            }
        }
    }

    let area = recursive_areas.get_mut(&level).unwrap();
    *area = result;
}

fn calculate_biodiversity(area: &[Vec<Tile>]) -> u64 {
    let mut sum = 0;
    for (y, row) in area.iter().enumerate() {
        for (x, _) in row.iter().enumerate() {
            let tile_number = y * SIZE + x;
            if area[y][x] == Tile::Bug {
                sum += 2_u64.pow(tile_number as u32);
            }
        }
    }

    sum
}

fn create_empty_area() -> Vec<Vec<Tile>> {
    let mut empty_area = vec![vec![Tile::Empty; SIZE]; SIZE];
    empty_area[2][2] = Tile::Subgrid;
    empty_area
}

fn part_1(mut area: Vec<Vec<Tile>>) {
    let mut ratings = HashSet::new();
    loop {
        area = process_part_1(&area);
        let rating = calculate_biodiversity(&area);
        if !ratings.insert(rating) {
            assert_eq!(rating, 32_523_825);
            break;
        }
    }
}

fn part_2(area: Vec<Vec<Tile>>) {
    let mut recursive_areas = HashMap::new();
    recursive_areas.insert(0, area);

    const MINUTES: i32 = 200;
    let mut added_level = 1;
    for iteration in 1..=MINUTES {
        recursive_areas.insert(-added_level, create_empty_area());
        recursive_areas.insert(added_level, create_empty_area());
        added_level += 1;

        process_part_2(&mut recursive_areas, iteration);
    }

    let mut sum = 0;
    for area in recursive_areas.values() {
        for row in area.iter() {
            sum += row.iter().filter(|v| **v == Tile::Bug).count();
        }
    }

    assert_eq!(sum, 2040);
}

fn main() {
    let area = vec![
        vec![Tile::Empty, Tile::Empty, Tile::Bug, Tile::Bug, Tile::Bug],
        vec![Tile::Empty, Tile::Bug, Tile::Bug, Tile::Bug, Tile::Bug],
        vec![
            Tile::Empty,
            Tile::Empty,
            Tile::Empty,
            Tile::Bug,
            Tile::Empty,
        ],
        vec![Tile::Empty, Tile::Bug, Tile::Empty, Tile::Empty, Tile::Bug],
        vec![Tile::Bug, Tile::Empty, Tile::Bug, Tile::Bug, Tile::Bug],
    ];

    part_1(area.clone());
    part_2(area);
}
