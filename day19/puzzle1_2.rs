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
    input: Vec<i64>,
    output: i64,
}

impl Computer {
    fn new(program: Vec<i64>) -> Self {
        let mut c = Computer {
            memory: program,
            pc: 0,
            relative_base: 0,
            input: Vec::new(),
            output: 0,
        };

        c.memory.resize(2000, 0);
        c
    }

    fn run(&mut self) -> i64 {
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
        }

        self.output
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
        let input = self.input.pop().unwrap();
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
        //println!("OUTPUT: {}", output);
        self.output = output;
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

    fn set_input(&mut self, x: i64, y: i64) {
    	self.input.push(y);
    	self.input.push(x);
    }
}

fn main() {
    let contents = fs::read_to_string("input").expect("file not found");
    let contents = contents.trim();

    let mut program = Vec::new();
    for value in contents.split(',') {
        program.push(value.parse::<i64>().unwrap());
    }

    // PART 1
    let mut sum = 0;
    for x in 0..50 {
    	for y in 0..50 {
    		let mut computer = Computer::new(program.clone());
    		computer.set_input(x as i64, y as i64);
    		sum += computer.run();
    	}
    }
    println!("{}", sum);

    // PART 2
	let ship_size: usize = 100;
	let ship_offset = ship_size - 1;
    let mut current_x = 0;
    let mut current_y = ship_size - 1;

    loop {
    	for x in current_x.. {
    		let mut computer = Computer::new(program.clone());
    		computer.set_input(x as i64, current_y as i64);
    		let output = computer.run();

    		if output == 1 {
    			current_x = x;
    			break;
    		}
    	}

    	let calculate_on_pos = |x: usize, y: usize| -> i64 {
			let mut computer = Computer::new(program.clone());
			computer.set_input(x as i64, y as i64);
			computer.run()
    	};
    	let bottom_left = calculate_on_pos(current_x, current_y);
    	let bottom_right = calculate_on_pos(current_x + ship_offset, current_y);
    	let top_left = calculate_on_pos(current_x, current_y - ship_offset);
    	let top_right = calculate_on_pos(current_x + ship_offset, current_y - ship_offset);

    	if bottom_left == 1 && bottom_right == 1 && top_left == 1 && top_right == 1 {
    		println!("{}", current_x * 10_000 + current_y - ship_offset);
    		break;
    	}
    	current_y += 1;
    }
}