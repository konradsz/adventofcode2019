use std::collections::HashSet;

#[derive(Clone, PartialEq)]
enum Tile {
    Bug,
    Empty,
}

const SIZE: usize = 5;

fn count_adjacent_bugs(area: &Vec<Vec<Tile>>, x: usize, y: usize) -> u32 {
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

fn process(area: &Vec<Vec<Tile>>) -> Vec<Vec<Tile>> {
    let mut result = vec![vec![Tile::Empty; SIZE]; SIZE];

    for (y, row) in area.iter().enumerate() {
        for (x, tile) in row.iter().enumerate() {
            let adjacent_bugs = count_adjacent_bugs(&area, x, y);
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
            }
        }
    }

    result
}

fn calculate_biodiversity(area: &Vec<Vec<Tile>>) -> u64 {
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

fn main() {
    let mut area = vec![
        vec![Tile::Empty, Tile::Empty, Tile::Bug, Tile::Bug, Tile::Bug],
        vec![Tile::Empty, Tile::Bug, Tile::Bug, Tile::Bug, Tile::Bug],
        vec![Tile::Empty, Tile::Empty, Tile::Empty, Tile::Bug, Tile::Empty],
        vec![Tile::Empty, Tile::Bug, Tile::Empty, Tile::Empty, Tile::Bug],
        vec![Tile::Bug, Tile::Empty, Tile::Bug, Tile::Bug, Tile::Bug],
    ];

    let mut ratings = HashSet::new();
    loop {
        area = process(&area);
        let rating = calculate_biodiversity(&area);
        if !ratings.insert(rating) {
            println!("{}", rating);
            break;
        }
    }
}
