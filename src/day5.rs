use crate::intcode::prelude::*;

fn day5_gen(input: &str) -> Vec<i64> {
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

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let mut helper = aoc_helper::Helper::new_with_serializer(2019, 5, day5_gen);
    helper.part1(part1);
    helper.part2(part2);
    helper.run()?;
    Ok(())
}
