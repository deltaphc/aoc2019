use aoc_runner_derive::{aoc, aoc_generator};
use crate::intcode::{Program, IOOperation};

#[aoc_generator(day5)]
fn day5_gen(input: &str) -> Vec<i32> {
    input
        .split(',')
        .flat_map(|num_str| num_str.parse::<i32>())
        .collect()
}

#[aoc(day5, part1)]
pub(crate) fn part1(input: &[i32]) -> i32 {
    let mut output = -6969;
    Program::from(input).run(|io_op| {
        match io_op {
            IOOperation::Input => return 1,
            IOOperation::Output(value) => {
                // Disabled for the sake of benchmarking
                //println!("{}", value);
                output = value;
                return 0;
            },
        }
    });
    output
}

#[aoc(day5, part2)]
pub(crate) fn part2(input: &[i32]) -> i32 {
    let mut output = -6969;
    Program::from(input).run(|io_op| {
        match io_op {
            IOOperation::Input => return 5,
            IOOperation::Output(value) => {
                // Disabled for the sake of benchmarking
                //println!("{}", value);
                output = value;
                return 0;
            },
        }
    });
    output
}
