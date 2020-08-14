use intcode::Intcode;
use std::fs;

struct Robot {
    intcode: Intcode,
}

impl Robot {
    fn new(program: &[isize]) -> Self {
        Self {
            intcode: Intcode::new(program),
        }
    }

    fn run(&mut self) {
        // A, B, A, B, A, C, A, C, B, C
        let main_routine = vec![
            65, 44, 66, 44, 65, 44, 66, 44, 65, 44, 67, 44, 65, 44, 67, 44, 66, 44, 67, 10,
        ];
        // R6, L10, R10, R10
        let fun_a = vec![
            82, 44, 54, 44, 76, 44, 49, 48, 44, 82, 44, 49, 48, 44, 82, 44, 49, 48, 10,
        ];
        // L10, L12, R10
        let fun_b = vec![76, 44, 49, 48, 44, 76, 44, 49, 50, 44, 82, 44, 49, 48, 10];
        // R6, L12, L10
        let fun_c = vec![82, 44, 54, 44, 76, 44, 49, 50, 44, 76, 44, 49, 48, 10];

        main_routine
            .iter()
            .chain(fun_a.iter())
            .chain(fun_b.iter())
            .chain(fun_c.iter())
            .for_each(|i| self.intcode.add_input(*i));

        self.intcode.add_input(110);
        self.intcode.add_input(10);

        self.intcode.write_to_memory(0, 2);
        self.intcode.run();

        let output_char: Vec<char> = self
            .intcode
            .get_output()
            .iter()
            .map(|value| *value as u8 as char)
            .collect();
        let mut grid: Vec<Vec<char>> = Vec::new();
        for row in output_char.split(|o| *o == '\n') {
            grid.push(row.to_vec());
        }

        let mut sum = 0;
        for (y, row) in grid.iter().enumerate() {
            if y == 0 || y > 45 {
                continue;
            }
            for (x, c) in row.iter().enumerate() {
                if x == 0 || x > 48 {
                    continue;
                }

                if *c == '#'
                    && grid[y - 1][x] == '#'
                    && grid[y + 1][x] == '#'
                    && grid[y][x - 1] == '#'
                    && grid[y][x + 1] == '#'
                {
                    sum += x * y;
                }
            }
        }

        assert_eq!(sum, 3_660);
        assert_eq!(self.intcode.get_output().back().unwrap(), &962_913);
    }
}

fn main() {
    let content = fs::read_to_string("input").expect("file not found");
    let content = content.trim();

    let program: Vec<_> = content
        .split(',')
        .map(|value| value.parse::<isize>().unwrap())
        .collect();

    let mut robot = Robot::new(&program);
    robot.run();
}
