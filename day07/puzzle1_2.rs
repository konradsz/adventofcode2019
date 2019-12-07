use std::collections::VecDeque;
use std::fs;

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

#[derive(Clone)]
struct Computer {
    memory: Vec<i32>,
    pc: usize,
    inputs: VecDeque<i32>,
    finished: bool,
}

impl Computer {
    fn new(program: Vec<i32>) -> Self {
        Computer {
            memory: program,
            pc: 0,
            inputs: VecDeque::new(),
            finished: false,
        }
    }

    fn run(&mut self) -> i32 {
        let output;
        loop {
            let (opcode, m1, m2) = self.decode_instruction();

            match opcode {
                1 => self.process_1(m1, m2),
                2 => self.process_2(m1, m2),
                3 => self.process_3(m1, m2),
                4 => {
                    self.process_4(m1, m2);
                    output = self.inputs.pop_front().unwrap();
                    break;
                }
                5 => self.process_5(m1, m2),
                6 => self.process_6(m1, m2),
                7 => self.process_7(m1, m2),
                8 => self.process_8(m1, m2),
                99 => {
                    if self.inputs.len() != 1 {
                        panic!("what? {}", self.inputs.len());
                    }

                    output = self.inputs.pop_front().unwrap();
                    self.finished = true;
                    break;
                }
                _ => (),
            }
        }
        output
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
        let input = self.inputs.pop_front().unwrap();
        let param = self.memory[self.pc + 1];
        self.memory[param as usize] = input;
        self.pc += 2;
    }

    fn process_4(&mut self, mode1: Mode, _: Mode) {
        let param = match mode1 {
            Mode::Position => self.memory[self.pc + 1],
            Mode::Immediate => (self.pc + 1) as i32,
        };

        let output = self.memory[param as usize];
        self.inputs.push_back(output);
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

    fn add_input(&mut self, input: i32) {
        self.inputs.push_back(input);
    }
}

struct System {
    amplifiers: Vec<Computer>,
}

impl System {
    fn new(program: Vec<i32>) -> Self {
        System {
            amplifiers: vec![Computer::new(program.clone()); 5],
        }
    }

    fn run(&mut self, configuration: &Vec<i32>) -> i32 {
        self.amplifiers
            .iter_mut()
            .zip(configuration.iter())
            .for_each(|(computer, value)| computer.add_input(*value));

        let mut next_input = 0;
        loop {
            for amp in self.amplifiers.iter_mut() {
                amp.add_input(next_input);
                next_input = amp.run();
                if amp.finished {
                    return next_input;
                }
            }
        }
    }
}

fn generate_configurations(output: &mut Vec<Vec<i32>>, input: &mut Vec<i32>, n: usize) {
    if n == 1 {
        output.push(input.to_vec());
    }
    for i in 0..n {
        input.swap(i, n - 1);
        generate_configurations(output, input, n - 1);
        input.swap(i, n - 1);
    }
}

fn main() {
    let contents = fs::read_to_string("input").expect("file not found");
    let contents = contents.trim();

    let mut program = Vec::new();
    for value in contents.split(',') {
        program.push(value.parse::<i32>().unwrap());
    }

    let mut configurations = Vec::new();
    generate_configurations(&mut configurations, &mut vec![0, 1, 2, 3, 4], 5);
    let max_output = configurations
        .iter()
        .map(|configuration| System::new(program.clone()).run(&configuration))
        .max()
        .unwrap();
    println!("{}", max_output);

    let mut configurations = Vec::new();
    generate_configurations(&mut configurations, &mut vec![5, 6, 7, 8, 9], 5);
    let max_output = configurations
        .iter()
        .map(|configuration| System::new(program.clone()).run(&configuration))
        .max()
        .unwrap();
    println!("{}", max_output);
}
