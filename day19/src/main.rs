use intcode::Intcode;
use std::fs;

fn part_1(program: &[isize]) {
    let mut sum = 0;
    for x in 0..50 {
        for y in 0..50 {
            let mut intcode = Intcode::new(program);

            intcode.add_input(x);
            intcode.add_input(y);

            intcode.run();

            sum += intcode.get_first_output().unwrap();
        }
    }

    assert_eq!(sum, 118);
}

fn part_2(program: &[isize]) {
    let ship_size: isize = 100;
    let ship_offset = ship_size - 1;
    let mut current_x = 0;
    let mut current_y = ship_size - 1;

    loop {
        for x in current_x.. {
            let mut intcode = Intcode::new(program);
            intcode.add_input(x);
            intcode.add_input(current_y);
            intcode.run();

            if intcode.get_first_output().unwrap() == 1 {
                current_x = x;
                break;
            }
        }

        let calculate_on_pos = |x: isize, y: isize| -> isize {
            let mut intcode = Intcode::new(program);
            intcode.add_input(x);
            intcode.add_input(y);
            intcode.run();
            intcode.get_first_output().unwrap()
        };
        let bottom_left = calculate_on_pos(current_x, current_y);
        let bottom_right = calculate_on_pos(current_x + ship_offset, current_y);
        let top_left = calculate_on_pos(current_x, current_y - ship_offset);
        let top_right = calculate_on_pos(current_x + ship_offset, current_y - ship_offset);

        if bottom_left == 1 && bottom_right == 1 && top_left == 1 && top_right == 1 {
            assert_eq!(current_x * 10_000 + current_y - ship_offset, 18_651_593);
            break;
        }
        current_y += 1;
    }
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
