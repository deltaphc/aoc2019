use aoc_runner_derive::{aoc, aoc_generator};
use crate::intcode::{Program, IOOperation, IOReturn, ExecuteAction};

#[aoc_generator(day9)]
fn day9_gen(input: &str) -> Vec<i64> {
    input
        .split(',')
        .flat_map(|num_str| num_str.trim().parse::<i64>())
        .collect()
}

#[aoc(day9, part1)]
pub fn part1(input: &[i64]) -> i64 {
    let mut prog = Program::from(input);
    let mut output = -6969;
    prog.run(|io_op| {
        match io_op {
            IOOperation::Input => IOReturn::Input(1),
            IOOperation::Output(value) => {
                output = value;
                IOReturn::Output(ExecuteAction::Continue)
            }
        }
    });
    output
}

#[aoc(day9, part2)]
pub fn part2(input: &[i64]) -> i64 {
    let mut prog = Program::from(input);
    let mut output = -6969;
    prog.run(|io_op| {
        match io_op {
            IOOperation::Input => IOReturn::Input(2),
            IOOperation::Output(value) => {
                output = value;
                IOReturn::Output(ExecuteAction::Continue)
            }
        }
    });
    output
}
