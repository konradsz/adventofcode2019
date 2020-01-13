use std::fs;
use std::collections::HashMap;
use std::collections::VecDeque;

enum Mode {
    Position,
    Immediate,
    Relative,
}

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

impl From<i64> for Mode {
    fn from(item: i64) -> Self {
        match item {
            0 => Mode::Position,
            1 => Mode::Immediate,
            2 => Mode::Relative,
            _ => panic!(),
        }
    }
}

struct Computer {
    memory: Vec<i64>,
    pc: usize,
    relative_base: i64,
    position: (i64, i64),
    area: HashMap<(i64, i64), Tile>,
    steps_record: Vec<Direction>,
    oxygen_system_position: (i64, i64),
    traversing_finished: bool,
}

impl Computer {
    fn new(program: Vec<i64>) -> Self {
        let mut c = Computer {
            memory: program,
            pc: 0,
            relative_base: 0,
            position: (0, 0),
            area: HashMap::new(),
            steps_record: Vec::new(),
            oxygen_system_position: (0, 0),
            traversing_finished: false,
        };

        c.area.insert(c.position, Tile::Start);

        c.memory.resize(2000, 0);
        c
    }

    fn run(&mut self) {
        loop {
            let (opcode, m1, m2, m3) = self.decode_instruction();

            match opcode {
                1 => self.process_1(m1, m2, m3),
                2 => self.process_2(m1, m2, m3),
                3 => self.process_3(m1, m2, m3),
                4 => self.process_4(m1, m2, m3),
                5 => self.process_5(m1, m2, m3),
                6 => self.process_6(m1, m2, m3),
                7 => self.process_7(m1, m2, m3),
                8 => self.process_8(m1, m2, m3),
                9 => self.process_9(m1, m2, m3),
                99 => break,
                _ => (),
            }

            if self.traversing_finished {
                if let Some(tile) = self.area.get_mut(&(0, 0)) {
                    *tile = Tile::Start;
                }
                if let Some(tile) = self.area.get_mut(&self.oxygen_system_position) {
                    *tile = Tile::Empty;
                }

                self.part1();
                self.part2();
                break;
            }
        }
    }

    fn decode_instruction(&self) -> (i64, Mode, Mode, Mode) {
        let value = self.memory[self.pc];
        let opcode = value % 100;
        let mode1 = (value % 1_000) / 100;
        let mode2 = (value % 10_000) / 1_000;
        let mode3 = value / 10_000;

        (
            opcode,
            Mode::from(mode1),
            Mode::from(mode2),
            Mode::from(mode3),
        )
    }

    fn decode_params(&self, mode1: Mode, mode2: Mode, mode3: Mode) -> (i64, i64, i64) {
        let param1 = match Mode::from(mode1) {
            Mode::Position => self.memory[self.memory[self.pc + 1] as usize],
            Mode::Immediate => self.memory[self.pc + 1],
            Mode::Relative => self.memory[(self.relative_base + self.memory[self.pc + 1]) as usize],
        };
        let param2 = match Mode::from(mode2) {
            Mode::Position => self.memory[self.memory[self.pc + 2] as usize],
            Mode::Immediate => self.memory[self.pc + 2],
            Mode::Relative => self.memory[(self.relative_base + self.memory[self.pc + 2]) as usize],
        };
        let param3 = match Mode::from(mode3) {
            Mode::Position => self.memory[self.pc + 3],
            Mode::Immediate => panic!(),
            Mode::Relative => (self.relative_base + self.memory[self.pc + 3]),
        };

        (param1, param2, param3)
    }

    fn process_1(&mut self, mode1: Mode, mode2: Mode, mode3: Mode) {
        let (param1, param2, param3) = self.decode_params(mode1, mode2, mode3);

        self.memory[param3 as usize] = param1 + param2;
        self.pc += 4;
    }

    fn process_2(&mut self, mode1: Mode, mode2: Mode, mode3: Mode) {
        let (param1, param2, param3) = self.decode_params(mode1, mode2, mode3);

        self.memory[param3 as usize] = param1 * param2;
        self.pc += 4;
    }

    fn process_3(&mut self, mode1: Mode, _: Mode, _: Mode) {
        let param = match mode1 {
            Mode::Position => self.memory[self.pc + 1],
            Mode::Immediate => (self.pc + 1) as i64,
            Mode::Relative => self.relative_base + self.memory[self.pc + 1],
        };

        let input = self.get_input();
        if input == -1 {
            self.traversing_finished = true;
            return;
        }
        self.memory[param as usize] = input;
        self.pc += 2;
    }

    fn process_4(&mut self, mode1: Mode, _: Mode, _: Mode) {
        let param = match mode1 {
            Mode::Position => self.memory[self.pc + 1],
            Mode::Immediate => (self.pc + 1) as i64,
            Mode::Relative => (self.relative_base + self.memory[self.pc + 1]),
        };

        let output = self.memory[param as usize];
        match output {
            0 => {
                self.area.insert(self.position, Tile::Wall);
                self.return_one_step();
            },
            1 => { self.area.insert(self.position, Tile::Empty); },
            2 => {
                self.oxygen_system_position = self.position;
                self.area.insert(self.position, Tile::OxygenSystem);
            },
            _ => panic!()
        }
        self.pc += 2;
    }

    fn process_5(&mut self, mode1: Mode, mode2: Mode, mode3: Mode) {
        let (param1, param2, _) = self.decode_params(mode1, mode2, mode3);

        if param1 != 0 {
            self.pc = param2 as usize;
        } else {
            self.pc += 3;
        }
    }

    fn process_6(&mut self, mode1: Mode, mode2: Mode, mode3: Mode) {
        let (param1, param2, _) = self.decode_params(mode1, mode2, mode3);

        if param1 == 0 {
            self.pc = param2 as usize;
        } else {
            self.pc += 3;
        }
    }

    fn process_7(&mut self, mode1: Mode, mode2: Mode, mode3: Mode) {
        let (param1, param2, param3) = self.decode_params(mode1, mode2, mode3);

        if param1 < param2 {
            self.memory[param3 as usize] = 1;
        } else {
            self.memory[param3 as usize] = 0;
        }

        self.pc += 4;
    }

    fn process_8(&mut self, mode1: Mode, mode2: Mode, mode3: Mode) {
        let (param1, param2, param3) = self.decode_params(mode1, mode2, mode3);

        if param1 == param2 {
            self.memory[param3 as usize] = 1;
        } else {
            self.memory[param3 as usize] = 0;
        }

        self.pc += 4;
    }

    fn process_9(&mut self, mode1: Mode, _: Mode, _: Mode) {
        let param = match mode1 {
            Mode::Position => self.memory[self.pc + 1],
            Mode::Immediate => (self.pc + 1) as i64,
            Mode::Relative => self.relative_base + self.memory[self.pc + 1],
        };

        let offset = self.memory[param as usize];
        self.relative_base += offset;

        self.pc += 2;
    }

    fn get_next_step(&mut self) -> Option<Direction> {
        //self.print_area();
        let current_position = self.position;
        if self.area.get(&(current_position.0, current_position.1 - 1)).is_none() {
            return Some(Direction::Up);
        } else if self.area.get(&(current_position.0 + 1, current_position.1)).is_none() {
            return Some(Direction::Right);
        } else if self.area.get(&(current_position.0, current_position.1 + 1)).is_none() {
            return Some(Direction::Down);
        } else if self.area.get(&(current_position.0 - 1, current_position.1)).is_none() {
            return Some(Direction::Left);
        }

        None
    }

    fn get_input(&mut self) -> i64 {
        match self.get_next_step() {
            Some(Direction::Up) => {
                self.position.1 -= 1;
                self.steps_record.push(Direction::Up);
                return 1;
            },
            Some(Direction::Right) => {
                self.position.0 += 1;
                self.steps_record.push(Direction::Right);
                return 4;
            },
            Some(Direction::Down) => {
                self.position.1 += 1;
                self.steps_record.push(Direction::Down);
                return 2;
            },
            Some(Direction::Left) => {
                self.position.0 -= 1;
                self.steps_record.push(Direction::Left);
                return 3;
            },
            None => {
                return self.return_one_step();
            },
        };
    }

    fn return_one_step(&mut self) -> i64 {
        let last_step = self.steps_record.pop();
        match last_step {
            Some(Direction::Up) => {
                self.position.1 += 1;
                return 2;
            },
            Some(Direction::Right) => {
                self.position.0 -= 1;
                return 3;
            },
            Some(Direction::Down) => {
                self.position.1 -= 1;
                return 1;
            },
            Some(Direction::Left) => {
                self.position.0 += 1;
                return 4;
            },
            None => {
                //panic!("notink")
                return -1;
            }
        }
    }

    fn print_area(&self) {
        let x_min = self.area.keys().map(|k| k.0).min().unwrap();
        let y_min = self.area.keys().map(|k| k.1).min().unwrap();
        let x_max = self.area.keys().map(|k| k.0).max().unwrap();
        let y_max = self.area.keys().map(|k| k.1).max().unwrap();

        let width = (x_max - x_min).abs() + 1;
        let height = (y_max - y_min).abs() + 1;

        let mut area_to_print = vec![vec![5; width as usize]; height as usize];

        for (position, tile) in self.area.iter() {
            match tile {
                Tile::Start => area_to_print[(position.1 - y_min) as usize][(position.0 - x_min ) as usize] = 0,
                Tile::Empty => area_to_print[(position.1 - y_min) as usize][(position.0 - x_min ) as usize] = 1,
                Tile::Wall => area_to_print[(position.1 - y_min) as usize][(position.0 - x_min ) as usize] = 2,
                Tile::OxygenSystem => area_to_print[(position.1 - y_min) as usize][(position.0 - x_min ) as usize] = 3,
                Tile::Traversed => area_to_print[(position.1 - y_min) as usize][(position.0 - x_min ) as usize] = 4,
            };
        }

        //area_to_print[(self.position.1 - y_min) as usize][(self.position.0 - x_min ) as usize] = 4;

        for row in area_to_print.iter() {
            for c in row.iter() {
                match *c {
                    0 => print!("S"),
                    1 => print!("."),
                    2 => print!("#"),
                    3 => print!("O"),
                    4 => print!(""),
                    5 => print!(" "),
                    _ => panic!(),
                }
            }
            print!("\n");
        }
    }

    fn part1(&mut self) {
        self.print_area();

        let mut traversal_map: HashMap<(i64, i64), char> = HashMap::new();
        let mut queue = VecDeque::new();
        queue.push_back((0, 0));

        while queue.len() != 0 {
            let current_position = queue.pop_front().unwrap();
            if current_position == self.oxygen_system_position {
                break;
            }

            let up = (current_position.0, current_position.1 - 1);
            if let Some(tile) = self.area.get_mut(&up) {
                if *tile == Tile::Empty {
                    queue.push_back(up);
                    *tile = Tile::Traversed;
                    traversal_map.insert(up, 'd');
                }
            }

            let down = (current_position.0, current_position.1 + 1);
            if let Some(tile) = self.area.get_mut(&down) {
                if *tile == Tile::Empty {
                    queue.push_back(down);
                    *tile = Tile::Traversed;
                    traversal_map.insert(down, 'g');
                }
            }

            let left = (current_position.0 - 1, current_position.1);
            if let Some(tile) = self.area.get_mut(&left) {
                if *tile == Tile::Empty {
                    queue.push_back(left);
                    *tile = Tile::Traversed;
                    traversal_map.insert(left, 'p');
                }
            }

            let right = (current_position.0 + 1, current_position.1);
            if let Some(tile) = self.area.get_mut(&right) {
                if *tile == Tile::Empty {
                    queue.push_back(right);
                    *tile = Tile::Traversed;
                    traversal_map.insert(right, 'l');
                }
            }
        }

        let mut traversal_position = self.oxygen_system_position;
        let mut steps = 0;
        loop {
            if traversal_position == (0, 0) {
                break;
            }
            steps += 1;
            let direction = traversal_map.get(&traversal_position).unwrap();
            match direction {
                'd' => traversal_position = (traversal_position.0, traversal_position.1 + 1),
                'g' => traversal_position = (traversal_position.0, traversal_position.1 - 1),
                'p' => traversal_position = (traversal_position.0 + 1, traversal_position.1),
                'l' => traversal_position = (traversal_position.0 - 1, traversal_position.1),
                _ => unreachable!()
            }
        }

        println!("Steps taken to oxygen system: {}", steps);

        for tile in self.area.values_mut() {
            if *tile == Tile::Traversed {
                *tile = Tile::Empty;
            }
        }
    }

    fn part2(&mut self) {
        let mut queue = VecDeque::new();

        let mut current_minute = 0;
        queue.push_back((self.oxygen_system_position, current_minute));

        while queue.len() != 0 {
            let current_element = queue.pop_front().unwrap();
            current_minute = current_element.1;
            let current_position = current_element.0;

            let up = (current_position.0, current_position.1 - 1);
            if let Some(tile) = self.area.get_mut(&up) {
                if *tile == Tile::Empty {
                    queue.push_back((up, current_minute + 1));
                    *tile = Tile::Traversed;
                }
            }

            let down = (current_position.0, current_position.1 + 1);
            if let Some(tile) = self.area.get_mut(&down) {
                if *tile == Tile::Empty {
                    queue.push_back((down, current_minute + 1));
                    *tile = Tile::Traversed;
                }
            }

            let left = (current_position.0 - 1, current_position.1);
            if let Some(tile) = self.area.get_mut(&left) {
                if *tile == Tile::Empty {
                    queue.push_back((left, current_minute + 1));
                    *tile = Tile::Traversed;
                }
            }

            let right = (current_position.0 + 1, current_position.1);
            if let Some(tile) = self.area.get_mut(&right) {
                if *tile == Tile::Empty {
                    queue.push_back((right, current_minute + 1));
                    *tile = Tile::Traversed;
                }
            }
        }

        println!("Minutes to fill the labyrinth: {}", current_minute);
    }
}

fn main() {
    let contents = fs::read_to_string("input").expect("file not found");
    let contents = contents.trim();

    let mut program = Vec::new();
    for value in contents.split(',') {
        program.push(value.parse::<i64>().unwrap());
    }

    let mut computer = Computer::new(program);
    computer.run();
}
