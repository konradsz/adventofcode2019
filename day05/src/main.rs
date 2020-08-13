use intcode::Intcode;
use std::fs;

fn part_1(program: &[isize]) {
    let mut intcode = Intcode::new(program);
    intcode.add_input(1);
    intcode.run();
    assert_eq!(Some(13_346_482), intcode.get_last_output());
}

fn part_2(program: &[isize]) {
    let mut intcode = Intcode::new(program);
    intcode.add_input(5);
    intcode.run();
    assert_eq!(Some(12_111_395), intcode.get_last_output());
}

fn main() {
    let content = fs::read_to_string("input").expect("file not found");
    let content = content.trim();

    let program: Vec<_> = content
        .split(',')
        .map(|value| value.parse::<isize>().unwrap())
        .collect();

    part_1(&program);
    part_2(&program);
}
