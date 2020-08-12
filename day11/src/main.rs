use intcode::Intcode;
use std::collections::{HashMap, HashSet};
use std::fs;

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Copy, PartialEq)]
enum Color {
    Black,
    White,
}

struct Robot {
    intcode: Intcode,
    position: (isize, isize),
    direction: Direction,
    grid: HashMap<(isize, isize), Color>,
    painted_tiles: HashSet<(isize, isize)>,
}

impl Robot {
    fn new(program: &[isize]) -> Self {
        Self {
            intcode: Intcode::new(program),
            position: (0, 0),
            direction: Direction::Up,
            grid: HashMap::new(),
            painted_tiles: HashSet::new(),
        }
    }

    fn run(&mut self) {
        //self.intcode.add_input(1);
        while !self.intcode.finished() {
            let input = match self.get_tile() {
                Color::Black => 0,
                Color::White => 1,
            };
            self.intcode.add_input(input);

            self.intcode.run();
            let output_1 = self.intcode.get_first_output().unwrap();
            let output_2 = self.intcode.get_first_output().unwrap();

            match output_1 {
                0 => *self.grid.get_mut(&self.position).unwrap() = Color::Black,
                1 => *self.grid.get_mut(&self.position).unwrap() = Color::White,
                _ => panic!(),
            };

            self.painted_tiles.insert(self.position);

            match output_2 {
                0 => self.turn_left(),
                1 => self.turn_right(),
                _ => panic!(),
            };
            self.move_one_step();
        }
    }

    fn get_tile(&mut self) -> Color {
        let entry = self.grid.entry(self.position).or_insert(Color::Black);
        *entry
    }

    fn turn_left(&mut self) {
        match self.direction {
            Direction::Up => self.direction = Direction::Left,
            Direction::Down => self.direction = Direction::Right,
            Direction::Left => self.direction = Direction::Down,
            Direction::Right => self.direction = Direction::Up,
        }
    }

    fn turn_right(&mut self) {
        match self.direction {
            Direction::Up => self.direction = Direction::Right,
            Direction::Down => self.direction = Direction::Left,
            Direction::Left => self.direction = Direction::Up,
            Direction::Right => self.direction = Direction::Down,
        }
    }

    fn move_one_step(&mut self) {
        match self.direction {
            Direction::Up => self.position.1 += 1,
            Direction::Down => self.position.1 -= 1,
            Direction::Left => self.position.0 -= 1,
            Direction::Right => self.position.0 += 1,
        }
    }

    fn paint_grid(&mut self) {
        let x_min = self.grid.keys().map(|k| k.0).min().unwrap();
        let y_min = self.grid.keys().map(|k| k.1).min().unwrap();
        let x_max = self.grid.keys().map(|k| k.0).max().unwrap();
        let y_max = self.grid.keys().map(|k| k.1).max().unwrap();

        let width = (x_max - x_min).abs() + 1;
        let height = (y_max - y_min).abs() + 1;

        let mut grid_to_print = vec![vec![0; width as usize]; height as usize];

        for (position, color) in self.grid.iter() {
            if *color == Color::White {
                grid_to_print[(position.1 - y_min) as usize][(position.0 - x_min) as usize] = 1;
            }
        }

        for row in grid_to_print.iter().rev() {
            for c in row.iter() {
                match *c {
                    1 => print!("*"),
                    _ => print!(" "),
                }
            }
            print!("\n");
        }
    }
}

fn part_1(program: &[isize]) {
    let mut robot = Robot::new(program);
    robot.run();

    assert_eq!(robot.painted_tiles.len(), 2373);
}

fn part_2(program: &[isize]) {
    let mut robot = Robot::new(program);
    robot.intcode.add_input(1);
    robot.run();

    robot.paint_grid();
}

fn main() {
    let contents = fs::read_to_string("input").expect("file not found");
    let contents = contents.trim();

    let mut program = Vec::new();
    for value in contents.split(',') {
        program.push(value.parse::<isize>().unwrap());
    }

    part_1(&program);
    part_2(&program);
}
