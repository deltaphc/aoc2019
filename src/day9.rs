use aoc_runner_derive::{aoc, aoc_generator};
use crate::intcode::{Program, IOOperation};

#[aoc_generator(day9)]
fn day9_gen(input: &str) -> Vec<i64> {
    input
        .split(',')
        .flat_map(|num_str| num_str.trim().parse::<i64>())
        .collect()
}

#[aoc(day9, part1)]
fn part1(input: &[i64]) -> i64 {
    let mut prog = Program::from(input);
    let mut output = -6969;
    prog.run(|io_op| {
        match io_op {
            IOOperation::Input => { return 1; },
            IOOperation::Output(value) => {
                output = value;
                return 0;
            }
        }
    });
    output
}

#[aoc(day9, part2)]
fn part2(input: &[i64]) -> i64 {
    let mut prog = Program::from(input);
    let mut output = -6969;
    prog.run(|io_op| {
        match io_op {
            IOOperation::Input => { return 2; },
            IOOperation::Output(value) => {
                output = value;
                return 0;
            }
        }
    });
    output
}
