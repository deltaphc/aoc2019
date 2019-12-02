use aoc_runner_derive::{aoc, aoc_generator};

#[aoc(day2, part1)]
fn part1(input: &str) -> usize {
    let mut program: Vec<usize> = input
        .split(',')
        .flat_map(|num| num.parse::<usize>().ok())
        .collect();

    program[1] = 12;
    program[2] = 2;

    for i in (0..program.len()).step_by(4) {
        match program[i] {
            1 => {
                let i1 = program[i + 1];
                let i2 = program[i + 2];
                let i3 = program[i + 3];
                program[i3] = program[i1] + program[i2];
            },
            2 => {
                let i1 = program[i + 1];
                let i2 = program[i + 2];
                let i3 = program[i + 3];
                program[i3] = program[i1] * program[i2];
            },
            99 => break,
            _ => panic!("Invalid opcode at position {}!", i),
        }
    }

    program[0]
}