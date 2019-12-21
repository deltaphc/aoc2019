use crate::intcode::prelude::*;

fn day5_gen(input: String) -> Vec<i64> {
    input
        .split(',')
        .flat_map(|num_str| num_str.parse::<i64>())
        .collect()
}

pub(crate) fn part1(input: Vec<i64>) -> i64 {
    let mut output = -6969;
    Program::from(input.as_slice()).run(|io_op| {
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

pub(crate) fn part2(input: Vec<i64>) -> i64 {
    let mut output = -6969;
    Program::from(input.as_slice()).run(|io_op| {
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

use aoc_helper::{AocDay, Puzzle};
pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let mut day = AocDay::new_with_serializer(2019, 5, day5_gen);
    let part1 = Puzzle::new(1, part1);
    let part2 = Puzzle::new(2, part2);
    day.run(&part1)?;
    day.run(&part2)?;
    Ok(())
}
