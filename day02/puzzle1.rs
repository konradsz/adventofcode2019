use std::fs;

struct Computer {
	memory: Vec<usize>,
	pc: usize
}

impl Computer {
	fn new(program: Vec<usize>) -> Self {
		Computer {
			memory: program,
			pc: 0
		}
	}

	fn run(&mut self) {
		loop {
			let opcode = self.memory[self.pc];
			match opcode {
				1 => self.process_1(),
				2 => self.process_2(),
				99 => break,
				_ => ()
			}
		}
	}

	fn process_1(&mut self) {
		let pos1 = self.memory[self.pc + 1];
		let pos2 = self.memory[self.pc + 2];
		let pos3 = self.memory[self.pc + 3];
		let result = self.memory[pos1] + self.memory[pos2];
		self.memory[pos3] = result;
		self.pc += 4;
	}

	fn process_2(&mut self) {
		let pos1 = self.memory[self.pc + 1];
		let pos2 = self.memory[self.pc + 2];
		let pos3 = self.memory[self.pc + 3];
		let result = self.memory[pos1] * self.memory[pos2];
		self.memory[pos3] = result;
		self.pc += 4;
	}

	fn get_value(&self, address: usize) -> usize {
		self.memory[address]
	}
}

fn main() {
	let contents = fs::read_to_string("input")
        .expect("file not found");
	let contents = contents.trim();

	let mut program = Vec::new();
	for value in contents.split(',') {
		program.push(value.parse::<usize>().unwrap());
	}

    program[1] = 12;
    program[2] = 2;

    let mut computer = Computer::new(program);
    computer.run();

    println!("{}", computer.get_value(0));
}
