use std::fs;

const WIDTH: usize = 25;
const HEIGHT: usize = 6;

#[derive(Clone, Copy, PartialEq)]
enum PixelType {
    Black = 0,
    White = 1,
    Transparent = 2,
}

fn part_1(content: &str) {
    let digits: Vec<u32> = content.chars().map(|c| c.to_digit(10).unwrap()).collect();

    let count_digits =
        |chunk: &[u32], expected| chunk.iter().filter(|digit| **digit == expected).count();

    let mut result = 0;
    let mut min_number_of_0s = std::usize::MAX;
    for chunk in digits.chunks(WIDTH * HEIGHT) {
        let n0 = count_digits(&chunk, 0);
        if n0 < min_number_of_0s {
            let n1 = count_digits(&chunk, 1);
            let n2 = count_digits(&chunk, 2);
            result = n1 * n2;
            min_number_of_0s = n0;
        }
    }

    assert_eq!(result, 1_792);
}

fn part_2(content: &str) {
    let digits: Vec<PixelType> = content
        .chars()
        .map(|c| match c.to_digit(10).unwrap() {
            0 => PixelType::Black,
            1 => PixelType::White,
            2 => PixelType::Transparent,
            _ => panic!(),
        })
        .collect();

    let mut image = [PixelType::Transparent; WIDTH * HEIGHT];

    for chunk in digits.chunks(WIDTH * HEIGHT) {
        for (index, digit) in chunk.iter().enumerate() {
            if image[index] == PixelType::Transparent {
                image[index] = *digit;
            }
        }
    }

    for (index, pixel) in image.iter().enumerate() {
        if index != 0 && index % WIDTH == 0 {
            println!();
        }
        match pixel {
            PixelType::Black => print!(" "),
            PixelType::White => print!("@"),
            PixelType::Transparent => print!(" "),
        }
    }
}

fn main() {
    let content = fs::read_to_string("input").expect("file not found");
    let content = content.trim();

    part_1(content);
    part_2(content);
}
