use std::collections::VecDeque;
use std::fs;

enum Mode {
    Position,
    Immediate,
    Relative,
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
    output: Vec<i64>,
    input: VecDeque<i64>,
}

impl Computer {
    fn new(program: Vec<i64>) -> Self {
        let mut c = Computer {
            memory: program,
            pc: 0,
            relative_base: 0,
            output: Vec::new(),
            input: VecDeque::new(),
        };

        c.memory.resize(4000, 0);
        c.memory[0] = 2;

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

        for n in main_routine
            .iter()
            .chain(fun_a.iter())
            .chain(fun_b.iter())
            .chain(fun_c.iter())
        {
            c.input.push_back(*n);
        }

        // n - no
        c.input.push_back(110);
        c.input.push_back(10);
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
                99 => {
                    self.calculate_alignment_parameters();
                    break;
                }
                _ => (),
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
        let input = self.input.pop_front().unwrap();
        self.memory[param as usize] = input;
        self.pc += 2;
    }

    fn process_4(&mut self, mode1: Mode, _: Mode, _: Mode) {
        let param = match mode1 {
            Mode::Position => self.memory[self.pc + 1],
            Mode::Immediate => (self.pc + 1) as i64,
            Mode::Relative => (self.relative_base + self.memory[self.pc + 1]),
        };

        self.output.push(self.memory[param as usize]);
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

    fn calculate_alignment_parameters(&self) {
        let output_char: Vec<char> = self
            .output
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

                if *c == '#' {
                    if grid[y - 1][x] == '#'
                        && grid[y + 1][x] == '#'
                        && grid[y][x - 1] == '#'
                        && grid[y][x + 1] == '#'
                    {
                        sum += x * y;
                    }
                }
            }
        }
        println!("Sum: {}", sum);
        println!("Collected dust: {}", self.output.last().unwrap());
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
