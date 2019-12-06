use std::fs;

const INPUT: i32 = 5;

enum Mode {
    Position,
    Immediate,
}

impl From<i32> for Mode {
    fn from(item: i32) -> Self {
        match item {
            0 => Mode::Position,
            1 => Mode::Immediate,
            _ => panic!(),
        }
    }
}

struct Computer {
    memory: Vec<i32>,
    pc: usize,
}

impl Computer {
    fn new(program: Vec<i32>) -> Self {
        Computer {
            memory: program,
            pc: 0,
        }
    }

    fn run(&mut self) {
        loop {
            let (opcode, m1, m2) = self.decode_instruction();

            match opcode {
                1 => self.process_1(m1, m2),
                2 => self.process_2(m1, m2),
                3 => self.process_3(m1, m2),
                4 => self.process_4(m1, m2),
                5 => self.process_5(m1, m2),
                6 => self.process_6(m1, m2),
                7 => self.process_7(m1, m2),
                8 => self.process_8(m1, m2),
                99 => break,
                _ => (),
            }
        }
    }

    fn decode_instruction(&self) -> (i32, Mode, Mode) {
        let value = self.memory[self.pc];
        let opcode = value % 100;
        let mode1 = (value % 1_000) / 100;
        let mode2 = (value % 10_000) / 1_000;

        (opcode, Mode::from(mode1), Mode::from(mode2))
    }

    fn decode_params(&self, mode1: Mode, mode2: Mode) -> (i32, i32) {
        let param1 = match Mode::from(mode1) {
            Mode::Position => self.memory[self.memory[self.pc + 1] as usize],
            Mode::Immediate => self.memory[self.pc + 1],
        };
        let param2 = match Mode::from(mode2) {
            Mode::Position => self.memory[self.memory[self.pc + 2] as usize],
            Mode::Immediate => self.memory[self.pc + 2],
        };

        (param1, param2)
    }

    fn process_1(&mut self, mode1: Mode, mode2: Mode) {
        let (param1, param2) = self.decode_params(mode1, mode2);
        let param3 = self.memory[self.pc + 3];

        self.memory[param3 as usize] = param1 + param2;
        self.pc += 4;
    }

    fn process_2(&mut self, mode1: Mode, mode2: Mode) {
        let (param1, param2) = self.decode_params(mode1, mode2);
        let param3 = self.memory[self.pc + 3];

        self.memory[param3 as usize] = param1 * param2;
        self.pc += 4;
    }

    fn process_3(&mut self, _: Mode, _: Mode) {
        let param = self.memory[self.pc + 1];
        self.memory[param as usize] = INPUT;
        self.pc += 2;
    }

    fn process_4(&mut self, mode1: Mode, _: Mode) {
        let param = match mode1 {
            Mode::Position => self.memory[self.pc + 1],
            Mode::Immediate => (self.pc + 1) as i32,
        };

        let output = self.memory[param as usize];
        println!("OUTPUT: {}", output);
        self.pc += 2;
    }

    fn process_5(&mut self, mode1: Mode, mode2: Mode) {
        let (param1, param2) = self.decode_params(mode1, mode2);

        if param1 != 0 {
            self.pc = param2 as usize;
        } else {
            self.pc += 3;
        }
    }

    fn process_6(&mut self, mode1: Mode, mode2: Mode) {
        let (param1, param2) = self.decode_params(mode1, mode2);

        if param1 == 0 {
            self.pc = param2 as usize;
        } else {
            self.pc += 3;
        }
    }

    fn process_7(&mut self, mode1: Mode, mode2: Mode) {
        let (param1, param2) = self.decode_params(mode1, mode2);
        let param3 = self.memory[self.pc + 3];

        if param1 < param2 {
            self.memory[param3 as usize] = 1;
        } else {
            self.memory[param3 as usize] = 0;
        }

        self.pc += 4;
    }

    fn process_8(&mut self, mode1: Mode, mode2: Mode) {
        let (param1, param2) = self.decode_params(mode1, mode2);
        let param3 = self.memory[self.pc + 3];

        if param1 == param2 {
            self.memory[param3 as usize] = 1;
        } else {
            self.memory[param3 as usize] = 0;
        }

        self.pc += 4;
    }
}

fn main() {
    let contents = fs::read_to_string("input").expect("file not found");
    let contents = contents.trim();

    let mut program = Vec::new();
    for value in contents.split(',') {
        program.push(value.parse::<i32>().unwrap());
    }

    let mut computer = Computer::new(program);
    computer.run();
}
