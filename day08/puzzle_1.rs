use std::fs;

const WIDTH: usize = 25;
const HEIGHT: usize = 6;

fn main() {
    let contents = fs::read_to_string("input").expect("file not found");
    let contents = contents.trim();

    let digits: Vec<u32> = contents.chars().map(|c| c.to_digit(10).unwrap()).collect();

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

    println!("{}", result);
}
