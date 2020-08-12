use intcode::Intcode;
use std::fs;

struct System {
    amplifiers: Vec<Intcode>,
}

impl System {
    fn new(program: &[isize]) -> Self {
        System {
            amplifiers: vec![Intcode::new(program); 5],
        }
    }

    fn run(&mut self, configuration: &[isize]) -> isize {
        self.amplifiers
            .iter_mut()
            .zip(configuration.iter())
            .for_each(|(amplifier, value)| amplifier.add_input(*value));

        let mut next_input = 0;
        loop {
            for amp in self.amplifiers.iter_mut() {
                amp.add_input(next_input);
                amp.run();
                if let Some(output) = amp.get_last_output() {
                    next_input = output;
                }
            }

            if self.amplifiers.last().unwrap().finished() {
                return next_input;
            }
        }
    }
}

fn generate_configurations(output: &mut Vec<Vec<isize>>, input: &mut Vec<isize>, n: usize) {
    if n == 1 {
        output.push(input.to_vec());
    }
    for i in 0..n {
        input.swap(i, n - 1);
        generate_configurations(output, input, n - 1);
        input.swap(i, n - 1);
    }
}

fn part_1(program: &[isize]) {
    let mut configurations = Vec::new();
    generate_configurations(&mut configurations, &mut vec![0, 1, 2, 3, 4], 5);
    let max_output = configurations
        .iter()
        .map(|configuration| System::new(&program).run(&configuration))
        .max()
        .unwrap();

    assert_eq!(max_output, 30_940);
}

fn part_2(program: &[isize]) {
    let mut configurations = Vec::new();
    generate_configurations(&mut configurations, &mut vec![5, 6, 7, 8, 9], 5);
    let max_output = configurations
        .iter()
        .map(|configuration| System::new(&program).run(&configuration))
        .max()
        .unwrap();

    assert_eq!(max_output, 76_211_147);
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
