use intcode::Intcode;
use std::fs;

fn part_1(program: &[isize]) {
    let mut intcode = Intcode::new(program);
    intcode.write_to_memory(1, 12);
    intcode.write_to_memory(2, 2);
    intcode.run();

    assert_eq!(intcode.read_from_memory(0), 3_306_701);
}

fn part_2(program: &[isize]) {
    const EXPECTED: isize = 19_690_720;

    for noun in 0..100 {
        for verb in 0..100 {
            let mut intcode = Intcode::new(&program);
            intcode.write_to_memory(1, noun);
            intcode.write_to_memory(2, verb);

            intcode.run();

            if intcode.read_from_memory(0) == EXPECTED {
                assert_eq!(100 * noun + verb, 7_621);
                return;
            }
        }
    }
}

fn main() {
    let contents = fs::read_to_string("input").expect("file not found");
    let contents = contents.trim();

    let mut program = Vec::new();
    for value in contents.split(',') {
        program.push(value.parse::<isize>().unwrap());
    }

    part_1(&program);
    part_2(&program);
}
