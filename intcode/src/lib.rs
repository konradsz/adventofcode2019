use std::collections::HashMap;
use std::collections::VecDeque;

enum Mode {
    Position,
    Immediate,
    Relative,
}

impl From<isize> for Mode {
    fn from(item: isize) -> Self {
        match item {
            0 => Mode::Position,
            1 => Mode::Immediate,
            2 => Mode::Relative,
            _ => panic!("unsupported mode"),
        }
    }
}

#[derive(Clone)]
pub struct Intcode {
    memory: HashMap<usize, isize>,
    pc: usize,
    relative_base: isize,
    input: VecDeque<isize>,
    output: VecDeque<isize>,
    finished: bool,
    paused: bool,
}

impl Intcode {
    pub fn new(program: &[isize]) -> Self {
        let mut initial_memory = HashMap::new();
        for (address, byte) in program.iter().enumerate() {
            initial_memory.insert(address, *byte);
        }

        Self {
            memory: initial_memory,
            pc: 0,
            relative_base: 0,
            input: VecDeque::new(),
            output: VecDeque::new(),
            finished: false,
            paused: false,
        }
    }

    pub fn run(&mut self) {
        self.paused = false;

        while !self.finished && !self.paused {
            let (opcode, mode_1, mode_2, mode_3) = self.decode_instruction();
            match opcode {
                1 => self.process_1(mode_1, mode_2, mode_3),
                2 => self.process_2(mode_1, mode_2, mode_3),
                3 => self.process_3(mode_1),
                4 => self.process_4(mode_1),
                5 => self.process_5(mode_1, mode_2),
                6 => self.process_6(mode_1, mode_2),
                7 => self.process_7(mode_1, mode_2, mode_3),
                8 => self.process_8(mode_1, mode_2, mode_3),
                9 => self.process_9(mode_1),
                99 => self.process_99(),
                _ => panic!("unsupported opcode"),
            }
        }
    }

    fn decode_instruction(&mut self) -> (isize, Mode, Mode, Mode) {
        let value = self.read_from_memory(self.pc);
        self.pc += 1;

        let opcode = value % 100;
        let mode_1 = (value % 1_000) / 100;
        let mode_2 = (value % 10_000) / 1_000;
        let mode_3 = value / 10_000;

        (
            opcode,
            Mode::from(mode_1),
            Mode::from(mode_2),
            Mode::from(mode_3),
        )
    }

    fn process_1(&mut self, mode_1: Mode, mode_2: Mode, mode_3: Mode) {
        let input_1 = self.read(mode_1);
        let input_2 = self.read(mode_2);
        let result = input_1 + input_2;
        self.write(mode_3, result);
    }

    fn process_2(&mut self, mode_1: Mode, mode_2: Mode, mode_3: Mode) {
        let input_1 = self.read(mode_1);
        let input_2 = self.read(mode_2);
        let result = input_1 * input_2;
        self.write(mode_3, result);
    }

    fn process_3(&mut self, mode: Mode) {
        if let Some(input) = self.input.pop_front() {
            self.write(mode, input);
        } else {
            self.pc -= 1;
            self.paused = true;
        }
    }

    fn process_4(&mut self, mode: Mode) {
        let output = self.read(mode);
        self.output.push_back(output);
    }

    fn process_5(&mut self, mode_1: Mode, mode_2: Mode) {
        let param_1 = self.read(mode_1);
        let param_2 = self.read(mode_2);

        if param_1 != 0 {
            self.pc = param_2 as usize;
        }
    }

    fn process_6(&mut self, mode_1: Mode, mode_2: Mode) {
        let param_1 = self.read(mode_1);
        let param_2 = self.read(mode_2);

        if param_1 == 0 {
            self.pc = param_2 as usize;
        }
    }

    fn process_7(&mut self, mode_1: Mode, mode_2: Mode, mode_3: Mode) {
        let param_1 = self.read(mode_1);
        let param_2 = self.read(mode_2);

        if param_1 < param_2 {
            self.write(mode_3, 1);
        } else {
            self.write(mode_3, 0);
        }
    }

    fn process_8(&mut self, mode_1: Mode, mode_2: Mode, mode_3: Mode) {
        let param_1 = self.read(mode_1);
        let param_2 = self.read(mode_2);

        if param_1 == param_2 {
            self.write(mode_3, 1);
        } else {
            self.write(mode_3, 0);
        }
    }

    fn process_9(&mut self, mode: Mode) {
        let offset = self.read(mode);
        self.relative_base += offset;
    }

    fn process_99(&mut self) {
        self.finished = true;
    }

    pub fn read_from_memory(&mut self, address: usize) -> isize {
        let entry = self.memory.entry(address).or_insert(0);
        *entry
    }

    pub fn write_to_memory(&mut self, address: usize, value: isize) {
        self.memory.insert(address, value);
    }

    fn read(&mut self, mode: Mode) -> isize {
        match mode {
            Mode::Position => {
                let address = self.read_from_memory(self.pc);
                self.pc += 1;
                let entry = self.memory.entry(address as usize).or_insert(0);
                *entry
            }
            Mode::Immediate => {
                let value = self.read_from_memory(self.pc);
                self.pc += 1;

                value
            }
            Mode::Relative => {
                let offset = self.read_from_memory(self.pc);
                self.pc += 1;
                let entry = self
                    .memory
                    .entry((self.relative_base + offset) as usize)
                    .or_insert(0);
                *entry
            }
        }
    }

    fn write(&mut self, mode: Mode, value: isize) {
        match mode {
            Mode::Position => {
                let address = self.read_from_memory(self.pc);
                self.pc += 1;
                self.write_to_memory(address as usize, value);
            }
            Mode::Immediate => {
                panic!("write in immediate mode should not happen");
            }
            Mode::Relative => {
                let offset = self.read_from_memory(self.pc);
                self.pc += 1;
                self.write_to_memory((self.relative_base + offset) as usize, value);
            }
        }
    }

    pub fn add_input(&mut self, value: isize) {
        self.input.push_back(value);
    }

    pub fn has_output(&self) -> bool {
        !self.output.is_empty()
    }

    pub fn get_output(&self) -> &VecDeque<isize> {
        &self.output
    }

    pub fn get_first_output(&mut self) -> Option<isize> {
        self.output.pop_front()
    }

    pub fn get_last_output(&mut self) -> Option<isize> {
        self.output.pop_back()
    }

    pub fn finished(&self) -> bool {
        self.finished
    }
}
