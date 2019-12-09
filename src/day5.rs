use aoc_runner_derive::{aoc, aoc_generator};
use crate::intcode::{Program, IOOperation, IOReturn, ExecuteAction};

#[aoc_generator(day5)]
fn day5_gen(input: &str) -> Vec<i64> {
    input
        .split(',')
        .flat_map(|num_str| num_str.parse::<i64>())
        .collect()
}

#[aoc(day5, part1)]
pub(crate) fn part1(input: &[i64]) -> i64 {
    let mut output = -6969;
    Program::from(input).run(|io_op| {
        match io_op {
            IOOperation::Input => IOReturn::Input(1),
            IOOperation::Output(value) => {
                // Disabled for the sake of benchmarking
                //println!("{}", value);
                output = value;
                IOReturn::Output(ExecuteAction::Continue)
            },
        }
    });
    output
}

#[aoc(day5, part2)]
pub(crate) fn part2(input: &[i64]) -> i64 {
    let mut output = -6969;
    Program::from(input).run(|io_op| {
        match io_op {
            IOOperation::Input => IOReturn::Input(5),
            IOOperation::Output(value) => {
                // Disabled for the sake of benchmarking
                //println!("{}", value);
                output = value;
                IOReturn::Output(ExecuteAction::Continue)
            },
        }
    });
    output
}
