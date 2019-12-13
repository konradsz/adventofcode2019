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
    output: Vec<String>,
}

impl Computer {
    fn new(program: Vec<i64>) -> Self {
        let mut c = Computer {
            memory: program,
            pc: 0,
            relative_base: 0,
            output: Vec::new(),
        };

        c.memory.resize(3000, 0);
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
                    self.print_scores();
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

        let input = Computer::determine_direction(self.get_ball_coordinates(), self.get_paddle_coordinates());
        if input != 0 {
            self.remove_old_paddle();
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
        self.output.push(output.to_string());
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

    fn get_block_tiles_count(&self) -> usize {
        self.output
            .iter()
            .skip(2)
            .step_by(3)
            .filter(|e| *e == "2")
            .count()
    }

    fn get_ball_coordinates(&mut self) -> (u32, u32) {
        for value in self.output.chunks_mut(3) {
            if &value[2] == "4" {
                value[2] = "0".to_string();
                return (value[0].parse::<u32>().unwrap(), value[1].parse::<u32>().unwrap());
            }
        }
        unreachable!()
    }

    fn get_paddle_coordinates(&mut self) -> (u32, u32) {
        for value in self.output.chunks_mut(3) {
            if &value[2] == "3" {
                return (value[0].parse::<u32>().unwrap(), value[1].parse::<u32>().unwrap());
            }
        }
        unreachable!()
    }

    fn remove_old_paddle(&mut self) {
        for value in self.output.chunks_mut(3) {
            if &value[2] == "3" {
                value[2] = "0".to_string();
            }
        }
    }

    fn determine_direction(ball: (u32, u32), paddle: (u32, u32)) -> i64 {
        if ball.0 == paddle.0 {
            return 0;
        } else if ball.0 > paddle.0 {
            return 1;
        } else if ball.0 < paddle.0 {
            return -1;
        }

        unreachable!()
    }

    fn print_scores(&self) {
        for value in self.output.chunks(3) {
            if &value[0] == "-1" && &value[1] == "0" {
                println!("{}", &value[2])
            }
        }
    }
}

fn main() {
    let contents = fs::read_to_string("input").expect("file not found");
    let contents = contents.trim();

    let mut program = Vec::new();
    for value in contents.split(',') {
        program.push(value.parse::<i64>().unwrap());
    }

    {
        let mut computer = Computer::new(program.clone());
        computer.run();
        println!("{}", computer.get_block_tiles_count());
    }

    {
        let mut computer = Computer::new(program.clone());
        computer.memory[0] = 2;
        computer.run();
    }
}
