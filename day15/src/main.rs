use intcode::Intcode;
use std::collections::{HashMap, VecDeque};
use std::fs;

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(PartialEq, Debug)]
enum Tile {
    Start,
    Empty,
    Wall,
    OxygenSystem,
    Traversed,
}

struct Droid {
    intcode: Intcode,
    position: (isize, isize),
    area: HashMap<(isize, isize), Tile>,
    steps_record: Vec<Direction>,
    oxygen_system_position: (isize, isize),
    traversing_finished: bool,
}

impl Droid {
    fn new(program: &[isize]) -> Self {
        let mut area = HashMap::new();
        area.insert((0, 0), Tile::Start);

        Self {
            intcode: Intcode::new(program),
            position: (0, 0),
            area,
            steps_record: Vec::new(),
            oxygen_system_position: (0, 0),
            traversing_finished: false,
        }
    }

    fn traverse(&mut self) {
        while !self.traversing_finished {
            let input = self.get_input();
            if input == -1 {
                self.traversing_finished = true;
            } else {
                self.intcode.add_input(input);
            }

            self.intcode.run();

            if let Some(output) = self.intcode.get_last_output() {
                match output {
                    0 => {
                        self.area.insert(self.position, Tile::Wall);
                        self.return_one_step();
                    }
                    1 => {
                        self.area.insert(self.position, Tile::Empty);
                    }
                    2 => {
                        self.oxygen_system_position = self.position;
                        self.area.insert(self.position, Tile::OxygenSystem);
                    }
                    _ => panic!(),
                }
            }
        }

        if let Some(tile) = self.area.get_mut(&(0, 0)) {
            *tile = Tile::Start;
        }
        if let Some(tile) = self.area.get_mut(&self.oxygen_system_position) {
            *tile = Tile::Empty;
        }
    }

    fn get_next_step(&mut self) -> Option<Direction> {
        let current_position = self.position;
        if self
            .area
            .get(&(current_position.0, current_position.1 - 1))
            .is_none()
        {
            return Some(Direction::Up);
        } else if self
            .area
            .get(&(current_position.0 + 1, current_position.1))
            .is_none()
        {
            return Some(Direction::Right);
        } else if self
            .area
            .get(&(current_position.0, current_position.1 + 1))
            .is_none()
        {
            return Some(Direction::Down);
        } else if self
            .area
            .get(&(current_position.0 - 1, current_position.1))
            .is_none()
        {
            return Some(Direction::Left);
        }

        None
    }

    fn get_input(&mut self) -> isize {
        match self.get_next_step() {
            Some(Direction::Up) => {
                self.position.1 -= 1;
                self.steps_record.push(Direction::Up);
                1
            }
            Some(Direction::Right) => {
                self.position.0 += 1;
                self.steps_record.push(Direction::Right);
                4
            }
            Some(Direction::Down) => {
                self.position.1 += 1;
                self.steps_record.push(Direction::Down);
                2
            }
            Some(Direction::Left) => {
                self.position.0 -= 1;
                self.steps_record.push(Direction::Left);
                3
            }
            None => self.return_one_step(),
        }
    }

    fn return_one_step(&mut self) -> isize {
        let last_step = self.steps_record.pop();
        match last_step {
            Some(Direction::Up) => {
                self.position.1 += 1;
                2
            }
            Some(Direction::Right) => {
                self.position.0 -= 1;
                3
            }
            Some(Direction::Down) => {
                self.position.1 -= 1;
                1
            }
            Some(Direction::Left) => {
                self.position.0 += 1;
                4
            }
            None => -1,
        }
    }
}

fn part_1(program: &[isize]) {
    let mut droid = Droid::new(program);
    droid.traverse();

    let mut traversal_map: HashMap<(isize, isize), char> = HashMap::new();
    let mut queue = VecDeque::new();
    queue.push_back((0, 0));

    while !queue.is_empty() {
        let current_position = queue.pop_front().unwrap();
        if current_position == droid.oxygen_system_position {
            break;
        }

        let up = (current_position.0, current_position.1 - 1);
        if let Some(tile) = droid.area.get_mut(&up) {
            if *tile == Tile::Empty {
                queue.push_back(up);
                *tile = Tile::Traversed;
                traversal_map.insert(up, 'd');
            }
        }

        let down = (current_position.0, current_position.1 + 1);
        if let Some(tile) = droid.area.get_mut(&down) {
            if *tile == Tile::Empty {
                queue.push_back(down);
                *tile = Tile::Traversed;
                traversal_map.insert(down, 'u');
            }
        }

        let left = (current_position.0 - 1, current_position.1);
        if let Some(tile) = droid.area.get_mut(&left) {
            if *tile == Tile::Empty {
                queue.push_back(left);
                *tile = Tile::Traversed;
                traversal_map.insert(left, 'r');
            }
        }

        let right = (current_position.0 + 1, current_position.1);
        if let Some(tile) = droid.area.get_mut(&right) {
            if *tile == Tile::Empty {
                queue.push_back(right);
                *tile = Tile::Traversed;
                traversal_map.insert(right, 'l');
            }
        }
    }

    let mut traversal_position = droid.oxygen_system_position;
    let mut steps = 0;
    while traversal_position != (0, 0) {
        let direction = traversal_map.get(&traversal_position).unwrap();
        match direction {
            'd' => traversal_position = (traversal_position.0, traversal_position.1 + 1),
            'u' => traversal_position = (traversal_position.0, traversal_position.1 - 1),
            'r' => traversal_position = (traversal_position.0 + 1, traversal_position.1),
            'l' => traversal_position = (traversal_position.0 - 1, traversal_position.1),
            _ => unreachable!(),
        }
        steps += 1;
    }

    assert_eq!(steps, 298);
}

fn part_2(program: &[isize]) {
    let mut droid = Droid::new(program);
    droid.traverse();

    let mut queue = VecDeque::new();

    let mut current_minute = 0;
    queue.push_back((droid.oxygen_system_position, current_minute));

    while !queue.is_empty() {
        let current_element = queue.pop_front().unwrap();
        current_minute = current_element.1;
        let current_position = current_element.0;

        let up = (current_position.0, current_position.1 - 1);
        if let Some(tile) = droid.area.get_mut(&up) {
            if *tile == Tile::Empty {
                queue.push_back((up, current_minute + 1));
                *tile = Tile::Traversed;
            }
        }

        let down = (current_position.0, current_position.1 + 1);
        if let Some(tile) = droid.area.get_mut(&down) {
            if *tile == Tile::Empty {
                queue.push_back((down, current_minute + 1));
                *tile = Tile::Traversed;
            }
        }

        let left = (current_position.0 - 1, current_position.1);
        if let Some(tile) = droid.area.get_mut(&left) {
            if *tile == Tile::Empty {
                queue.push_back((left, current_minute + 1));
                *tile = Tile::Traversed;
            }
        }

        let right = (current_position.0 + 1, current_position.1);
        if let Some(tile) = droid.area.get_mut(&right) {
            if *tile == Tile::Empty {
                queue.push_back((right, current_minute + 1));
                *tile = Tile::Traversed;
            }
        }
    }

    assert_eq!(current_minute, 346);
}

fn main() {
    let content = fs::read_to_string("input").expect("file not found");
    let content = content.trim();

    let program: Vec<_> = content
        .split(',')
        .map(|value| value.parse::<isize>().unwrap())
        .collect();

    part_1(&program);
    part_2(&program);
}
