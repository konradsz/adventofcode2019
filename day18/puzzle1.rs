use std::collections::HashMap;
use std::collections::VecDeque;
use std::fs;

enum Tile {
    Wall,
    Empty,
    Key(char),
    Door(char),
}

#[derive(Clone, Hash, Eq, PartialEq)]
struct State {
    position: (usize, usize),
    keys_collected: u32,
}

impl State {
    fn new(position: (usize, usize), keys_collected: u32) -> Self {
        State {
            position,
            keys_collected,
        }
    }
}

fn main() {
    let content = fs::read_to_string("input").expect("file not found");

    let mut map: Vec<Vec<Tile>> = Vec::new();
    let mut starting_position: (usize, usize) = (0, 0);

    let mut all_keys = 0;
    for (y, line) in content.lines().enumerate() {
        map.push(Vec::new());
        for (x, tile) in line.chars().enumerate() {
            if tile.is_alphabetic() {
                if tile.is_lowercase() {
                    all_keys |= 1 << (tile as u8 - 'a' as u8);
                    map[y].push(Tile::Key(tile));
                } else if tile.is_uppercase() {
                    map[y].push(Tile::Door(tile));
                }
            } else if tile == '#' {
                map[y].push(Tile::Wall);
            } else if tile == '.' {
                map[y].push(Tile::Empty);
            } else if tile == '@' {
                map[y].push(Tile::Empty);
                starting_position = (x, y);
            }
        }
    }

    let initial_state = State::new(starting_position, 0);
    let mut queue: VecDeque<State> = VecDeque::new();
    queue.push_back(initial_state.clone());

    let mut steps_taken_to_state: HashMap<State, u32> = HashMap::new();
    steps_taken_to_state.insert(initial_state, 0);

    let directions = [(0, -1), (0, 1), (-1, 0), (1, 0)];

    let get_tile = |position: (usize, usize), state: &State| -> Option<Tile> {
        match map[position.1][position.0] {
            Tile::Empty => return Some(Tile::Empty),
            Tile::Key(c) => {
                if (state.keys_collected >> (c as u8 - 'a' as u8)) & 1 == 1 {
                    return Some(Tile::Empty);
                } else {
                    return Some(Tile::Key(c));
                }
            }
            Tile::Door(c) => {
                if (state.keys_collected >> (c as u8 + 32 - 'a' as u8)) & 1 == 1 {
                    return Some(Tile::Empty);
                } else {
                    return None;
                }
            }
            Tile::Wall => return None,
        }
    };

    while let Some(state) = queue.pop_front() {
        if state.keys_collected == all_keys {
            let steps = steps_taken_to_state.get(&state).unwrap();
            println!("{}", steps);
            break;
        }

        for direction in directions.iter() {
            let new_position = (
                (state.position.0 as i32 + direction.0) as usize,
                (state.position.1 as i32 + direction.1) as usize,
            );

            if let Some(steps) = steps_taken_to_state.get(&state) {
                let steps = *steps;
                if let Some(tile) = get_tile(new_position, &state) {
                    match tile {
                        Tile::Empty => {
                            let new_state = State::new(new_position, state.keys_collected);
                            if !steps_taken_to_state.contains_key(&new_state) {
                                steps_taken_to_state.insert(new_state.clone(), steps + 1);
                                queue.push_back(new_state.clone());
                            }
                        }
                        Tile::Key(c) => {
                            let keys = state.keys_collected | (1 << (c as u8 - 'a' as u8));
                            let new_state = State::new(new_position, keys);
                            steps_taken_to_state.insert(new_state.clone(), steps + 1);
                            queue.push_back(new_state.clone());
                        }
                        _ => unreachable!(),
                    }
                }
            }
        }
    }
}
