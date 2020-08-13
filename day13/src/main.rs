use intcode::Intcode;
//use std::collections::{HashMap, HashSet};
use std::fs;

fn part_1(program: &[isize]) {
    let mut intcode = Intcode::new(&program);
    intcode.run();

    let output = intcode.get_output();
    let block_count = output
        .iter()
        .skip(2)
        .step_by(3)
        .filter(|o| **o == 2)
        .count();
    assert_eq!(block_count, 329);
}

fn determine_direction(ball: (isize, isize), paddle: (isize, isize)) -> isize {
    if ball.0 == paddle.0 {
        return 0;
    } else if ball.0 > paddle.0 {
        return 1;
    } else if ball.0 < paddle.0 {
        return -1;
    }

    unreachable!()
}

fn part_2(program: &[isize]) {
    let mut intcode = Intcode::new(&program);
    intcode.write_to_memory(0, 2);

    let mut ball_position = (0, 0);
    let mut paddle_position = (0, 0);
    let mut score = 0;

    while !intcode.finished() {
        intcode.run();

        while intcode.has_output() {
            let tile_type = intcode.get_last_output().unwrap();
            let coord_y = intcode.get_last_output().unwrap();
            let coord_x = intcode.get_last_output().unwrap();

            if tile_type == 4 {
                ball_position = (coord_x, coord_y);
            } else if tile_type == 3 {
                paddle_position = (coord_x, coord_y);
            }

            if coord_x == -1 {
                score = tile_type;
            }
        }

        let input = determine_direction(ball_position, paddle_position);
        intcode.add_input(input);
    }

    assert_eq!(score, 15_973);
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
