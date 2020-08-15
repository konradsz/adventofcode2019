fn meets_criteria(password: u32, criteria: &dyn Fn(&Vec<u32>) -> bool) -> bool {
    let digits = vec![
        password / 100_000,
        (password % 100_000) / 10_000,
        (password % 10_000) / 1_000,
        (password % 1_000) / 100,
        (password % 100) / 10,
        password % 10,
    ];

    criteria(&digits)
}

fn part_1() {
    let criteria = |digits: &Vec<u32>| {
        digits.windows(2).all(|digits| digits[0] <= digits[1])
            && digits.windows(2).any(|digits| digits[0] == digits[1])
    };
    let counter = (347312..805915)
        .filter(|password| meets_criteria(*password, &criteria))
        .count();

    assert_eq!(counter, 594);
}

fn part_2() {
    let criteria = |digits: &Vec<u32>| {
        digits.windows(2).all(|digits| digits[0] <= digits[1])
            && digits
                .iter()
                .any(|digit| digits.iter().filter(|&d| d == digit).count() == 2)
    };
    let counter = (347312..805915)
        .filter(|password| meets_criteria(*password, &criteria))
        .count();

    assert_eq!(counter, 364);
}

fn main() {
    part_1();
    part_2();
}
