use intcode::Intcode;
use std::fs;

fn part_1(program: &[isize]) {
    let mut intcode = Intcode::new(&program);
    intcode.add_input(1);

    intcode.run();
    assert_eq!(Some(4288078517), intcode.get_last_output());
}

fn part_2(program: &[isize]) {
    let mut intcode = Intcode::new(&program);
    intcode.add_input(2);

    intcode.run();
    assert_eq!(Some(69256), intcode.get_last_output());
}

fn main() {
    let content = fs::read_to_string("input").expect("file not found");
    let content = content.trim();

    let mut program = Vec::new();
    for value in content.split(',') {
        program.push(value.parse::<isize>().unwrap());
    }

    part_1(&program);
    part_2(&program);
}
