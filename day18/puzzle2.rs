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
    let mut starting_positions: Vec<(usize, usize)> = Vec::new();

    let mut keys: HashMap<(usize, usize), char> = HashMap::new();

    for (y, line) in content.lines().enumerate() {
        map.push(Vec::new());
        for (x, tile) in line.chars().enumerate() {
            if tile.is_alphabetic() {
                if tile.is_lowercase() {
                    map[y].push(Tile::Key(tile));
                    keys.insert((x, y), tile);
                } else if tile.is_uppercase() {
                    map[y].push(Tile::Door(tile));
                }
            } else if tile == '#' {
                map[y].push(Tile::Wall);
            } else if tile == '.' {
                map[y].push(Tile::Empty);
            } else if tile == '@' {
                map[y].push(Tile::Empty);
                starting_positions.push((x, y));
            }
        }
    }

    let get_tile =
        |position: (usize, usize), state: &State, keys_to_collect: &Vec<char>| -> Option<Tile> {
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
                    //println!("lookig for {} in {:?}", (c as u8 + 32) as char, keys_to_collect);
                    if keys_to_collect.contains(&((c as u8 + 32) as char)) {
                        if (state.keys_collected >> (c as u8 + 32 - 'a' as u8)) & 1 == 1 {
                            return Some(Tile::Empty);
                        } else {
                            return None;
                        }
                    } else {
                        return Some(Tile::Empty);
                    }
                }
                Tile::Wall => return None,
            }
        };

    let min_x = starting_positions
        .iter()
        .min_by(|(x_1, _), (x_2, _)| x_1.cmp(x_2))
        .unwrap()
        .0;
    let max_x = starting_positions
        .iter()
        .max_by(|(x_1, _), (x_2, _)| x_1.cmp(x_2))
        .unwrap()
        .0;
    let min_y = starting_positions
        .iter()
        .min_by(|(_, y_1), (_, y_2)| y_1.cmp(y_2))
        .unwrap()
        .1;
    let max_y = starting_positions
        .iter()
        .max_by(|(_, y_1), (_, y_2)| y_1.cmp(y_2))
        .unwrap()
        .1;

    let top_left_keys: Vec<char> = keys
        .iter()
        .filter(|((x, y), _)| *x <= min_x && *y <= min_y)
        .map(|(_, key)| *key)
        .collect();
    let top_right_keys: Vec<char> = keys
        .iter()
        .filter(|((x, y), _)| *x >= max_x && *y <= min_y)
        .map(|(_, key)| *key)
        .collect();
    let bottom_left_keys: Vec<char> = keys
        .iter()
        .filter(|((x, y), _)| *x <= min_x && *y >= max_y)
        .map(|(_, key)| *key)
        .collect();
    let bottom_right_keys: Vec<char> = keys
        .iter()
        .filter(|((x, y), _)| *x >= max_x && *y >= max_y)
        .map(|(_, key)| *key)
        .collect();

    let mut total_steps = 0;
    for starting_position in starting_positions.iter() {
        let mut current_robot_steps = 0;
        let mut current_robot_keys = 0;

        let keys_to_collect = if starting_position.0 == min_x && starting_position.1 == min_y {
            &top_left_keys
        } else if starting_position.0 == max_x && starting_position.1 == min_y {
            &top_right_keys
        } else if starting_position.0 == min_x && starting_position.1 == max_y {
            &bottom_left_keys
        } else if starting_position.0 == max_x && starting_position.1 == max_y {
            &bottom_right_keys
        } else {
            unreachable!();
        };

        let initial_state = State::new(*starting_position, 0);
        let mut queue: VecDeque<State> = VecDeque::new();
        queue.push_back(initial_state.clone());

        let mut steps_taken_to_state: HashMap<State, u32> = HashMap::new();
        steps_taken_to_state.insert(initial_state, 0);

        let directions = [(0, -1), (0, 1), (-1, 0), (1, 0)];
        while let Some(state) = queue.pop_front() {
            for direction in directions.iter() {
                let new_position = (
                    (state.position.0 as i32 + direction.0) as usize,
                    (state.position.1 as i32 + direction.1) as usize,
                );

                if let Some(steps) = steps_taken_to_state.get(&state) {
                    let steps = *steps;
                    if let Some(tile) = get_tile(new_position, &state, keys_to_collect) {
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
                                if keys > current_robot_keys {
                                    current_robot_keys = keys;
                                    current_robot_steps = steps + 1;
                                }
                            }
                            _ => unreachable!(),
                        }
                    }
                }
            }
        }
        total_steps += current_robot_steps;
    }
    println!("{}", total_steps);
}
