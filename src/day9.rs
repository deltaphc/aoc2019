use crate::intcode::prelude::*;

fn day9_gen(input: String) -> Vec<i64> {
    input
        .split(',')
        .flat_map(|num_str| num_str.trim().parse::<i64>())
        .collect()
}

pub fn part1(input: Vec<i64>) -> i64 {
    let mut prog = Program::from(input.as_slice());
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

pub fn part2(input: Vec<i64>) -> i64 {
    let mut prog = Program::from(input.as_slice());
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

use aoc_helper::{AocDay, Puzzle};
pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let mut day = AocDay::new_with_serializer(2019, 9, day9_gen);
    let part1 = Puzzle::new(1, part1);
    let part2 = Puzzle::new(2, part2);
    day.run(&part1)?;
    day.run(&part2)?;
    Ok(())
}
