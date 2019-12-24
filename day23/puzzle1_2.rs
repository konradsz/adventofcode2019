use std::cell::RefCell;
use std::collections::VecDeque;
use std::fs;
use std::rc::Rc;

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
    input: VecDeque<i64>,
    output: Vec<i64>,
    router: Rc<RefCell<Router>>,
    idle_count: u32,
}

impl Computer {
    fn new(program: Vec<i64>, router: Rc<RefCell<Router>>) -> Self {
        let mut c = Computer {
            memory: program,
            pc: 0,
            relative_base: 0,
            input: VecDeque::new(),
            output: Vec::new(),
            router,
            idle_count: 0,
        };

        c.memory.resize(3000, 0);
        c
    }

    fn execute_one_instruction(&mut self) {
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
            99 => println!("99!"),
            _ => (),
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

        let input = self.input.pop_front();
        let mut input_value = -1;
        if input.is_some() {
            self.idle_count = 0;
            input_value = input.unwrap();
        } else {
            self.idle_count += 1;
        }
        self.memory[param as usize] = input_value;
        self.pc += 2;
    }

    fn process_4(&mut self, mode1: Mode, _: Mode, _: Mode) {
        let param = match mode1 {
            Mode::Position => self.memory[self.pc + 1],
            Mode::Immediate => (self.pc + 1) as i64,
            Mode::Relative => (self.relative_base + self.memory[self.pc + 1]),
        };

        let output = self.memory[param as usize];
        self.output.push(output);
        if self.output.len() == 3 {
            let packet = Packet::new(self.output[0], self.output[1], self.output[2]);
            println!(
                "Sending packet to: {}, x: {}, y: {}",
                self.output[0], self.output[1], self.output[2]
            );
            self.router.borrow_mut().send_packet(packet);
            self.output.clear();
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

    fn set_input(&mut self, value: i64) {
        self.input.push_back(value);
    }
}

#[derive(Clone)]
struct Packet {
    address: i64,
    x: i64,
    y: i64,
}

impl Packet {
    fn new(address: i64, x: i64, y: i64) -> Self {
        Packet { address, x, y }
    }
}

struct Router {
    packets: Vec<Packet>,
}

impl Router {
    fn new() -> Self {
        Router {
            packets: Vec::new(),
        }
    }

    fn send_packet(&mut self, packet: Packet) {
        self.packets.push(packet);
    }

    fn has_packets(&self) -> bool {
        self.packets.len() > 0
    }
}

fn main() {
    let contents = fs::read_to_string("input").expect("file not found");
    let contents = contents.trim();

    let mut program = Vec::new();
    for value in contents.split(',') {
        program.push(value.parse::<i64>().unwrap());
    }

    let mut network = Vec::new();

    let router = Rc::new(RefCell::new(Router::new()));
    for address in 0..50 {
        let mut computer = Computer::new(program.clone(), router.clone());
        computer.set_input(address);
        network.push(computer);
    }

    let mut nat_packet = Some(Packet::new(0, 0, 0));

    let mut y_sent_by_nat = VecDeque::new();
    'outer: loop {
        for computer in network.iter_mut() {
            computer.execute_one_instruction();
        }

        let has_packets = router.borrow().has_packets();
        if has_packets {
            let packets = router.borrow().packets.clone();
            for packet in packets {
                let address = packet.address as usize;
                if address == 255 {
                    nat_packet = Some(Packet::new(0, packet.x, packet.y));
                } else {
                    network[address].set_input(packet.x);
                    network[address].set_input(packet.y);
                }
            }
            router.borrow_mut().packets.clear();
        }

        let idle = network.iter().all(|c| c.idle_count > 2);
        if idle && nat_packet.is_some() {
            let packet = nat_packet.unwrap();
            let address = packet.address as usize;
            network[address].set_input(packet.x);
            network[address].set_input(packet.y);
            nat_packet = None;

            y_sent_by_nat.push_back(packet.y);
            if y_sent_by_nat.len() == 2 {
                if y_sent_by_nat[0] == y_sent_by_nat[1] {
                    println!("Sent twice: {}", y_sent_by_nat[0]);
                    break 'outer;
                } else {
                    y_sent_by_nat.pop_front();
                }
            }
        }
    }
}
