use std::fs;

const WIDTH: usize = 25;
const HEIGHT: usize = 6;

#[derive(Clone, Copy, PartialEq)]
enum PixelType {
    Black = 0,
    White = 1,
    Transparent = 2,
}

fn main() {
    let contents = fs::read_to_string("input").expect("file not found");
    let contents = contents.trim();

    let digits: Vec<PixelType> = contents
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

    for _ in 0..=WIDTH {
        print!("@");
    }

    for (index, pixel) in image.iter().enumerate() {
        if index % WIDTH == 0 {
            print!("\n@");
        }
        match pixel {
            PixelType::Black => print!("@"),
            PixelType::White => print!("."),
            PixelType::Transparent => print!(" "),
        }
    }

    print!("\n");
    for _ in 0..=WIDTH {
        print!("@");
    }
}
