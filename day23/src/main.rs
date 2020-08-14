use intcode::Intcode;
use std::{cell::RefCell, collections::VecDeque, fs, rc::Rc};

struct Computer {
    intcode: Intcode,
    router: Rc<RefCell<Router>>,
}

impl Computer {
    fn new(program: &[isize], router: Rc<RefCell<Router>>) -> Self {
        Self {
            intcode: Intcode::new(program),
            router,
        }
    }
}

#[derive(Clone)]
struct Packet {
    address: isize,
    x: isize,
    y: isize,
}

impl Packet {
    fn new(address: isize, x: isize, y: isize) -> Self {
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
        !self.packets.is_empty()
    }
}

fn main() {
    let content = fs::read_to_string("input").expect("file not found");
    let content = content.trim();

    let program: Vec<_> = content
        .split(',')
        .map(|value| value.parse::<isize>().unwrap())
        .collect();

    let mut network = Vec::new();

    let router = Rc::new(RefCell::new(Router::new()));
    for address in 0..50 {
        let mut computer = Computer::new(&program, router.clone());
        computer.intcode.add_input(address);
        network.push(computer);
    }

    let mut idle_counter = vec![0; 50];
    let mut nat_packet = Some(Packet::new(0, 0, 0));

    let mut y_sent_by_nat = VecDeque::new();
    'outer: loop {
        for (address, computer) in network.iter_mut().enumerate() {
            if computer.intcode.awaits_input() {
                computer.intcode.add_input(-1);
                idle_counter[address] += 1;
            }

            computer.intcode.execute_single_instruction();

            if computer.intcode.get_output().len() >= 3 {
                let address = computer.intcode.get_first_output().unwrap();
                let x = computer.intcode.get_first_output().unwrap();
                let y = computer.intcode.get_first_output().unwrap();

                let packet = Packet::new(address, x, y);
                println!("Sending packet to: {}, x: {}, y: {}", address, x, y);
                computer.router.borrow_mut().send_packet(packet);
            }
        }

        let has_packets = router.borrow().has_packets();
        if has_packets {
            let packets = router.borrow().packets.clone();
            for packet in packets {
                let address = packet.address as usize;
                if address == 255 {
                    nat_packet = Some(Packet::new(0, packet.x, packet.y));
                } else {
                    network[address].intcode.add_input(packet.x);
                    network[address].intcode.add_input(packet.y);

                    idle_counter[address] = 0;
                }
            }
            router.borrow_mut().packets.clear();
        }

        let is_network_idle = idle_counter.iter().all(|c| c > &2);
        if is_network_idle && nat_packet.is_some() {
            let packet = nat_packet.unwrap();
            let address = packet.address as usize;
            network[address].intcode.add_input(packet.x);
            network[address].intcode.add_input(packet.y);
            nat_packet = None;

            y_sent_by_nat.push_back(packet.y);
            if y_sent_by_nat.len() == 2 {
                if y_sent_by_nat[0] == y_sent_by_nat[1] {
                    println!("Sent twice: {}", y_sent_by_nat[0]);
                    assert_eq!(y_sent_by_nat[0], 15_742);
                    break 'outer;
                } else {
                    y_sent_by_nat.pop_front();
                }
            }
        }
    }
}
