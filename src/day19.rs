use crate::intcode::prelude::*;

fn day19_gen(input: &str) -> Vec<i64> {
    input
        .split(',')
        .flat_map(|num_str| num_str.trim().parse::<i64>())
        .collect()
}

fn part1(input: Vec<i64>) -> i64 {
    let mut prog = Program::from(input.as_slice());
    let mut total = 0;
    for y in 0..50 {
        for x in 0..50 {
            let mut input_iter = std::iter::once(x).chain(std::iter::once(y));
            prog.reset();
            prog.run(|io_op| {
                match io_op {
                    IOOperation::Input => IOReturn::Input(input_iter.next().unwrap()),
                    IOOperation::Output(value) => {
                        match value {
                            1 => total += 1,
                            _ => (),
                        }
                        IOReturn::Output(ExecuteAction::Continue)
                    },
                }
            });
        }
    }
    total
}

fn in_tractor_beam(input: &[i64], x: i64, y: i64) -> bool {
    let mut prog = Program::from(input);
    let mut input_iter = std::iter::once(x).chain(std::iter::once(y));
    let mut output = false;
    prog.run(|io_op| {
        match io_op {
            IOOperation::Input => IOReturn::Input(input_iter.next().unwrap()),
            IOOperation::Output(value) => {
                match value {
                    0 => output = false,
                    1 => output = true,
                    _ => (),
                }
                IOReturn::Output(ExecuteAction::Break)
            },
        }
    });
    output
}

fn part2(input: Vec<i64>) -> i64 {
    let mut x_left = 0;
    let mut found_left = false;
    for y in 10.. { // start at 10 because input has first few missing
        'x_loop: for x in x_left.. {
            if in_tractor_beam(&input, x, y) {
                if !found_left {
                    x_left = x;
                    found_left = true;
                }

                if in_tractor_beam(&input, x + 99, y) && in_tractor_beam(&input, x, y + 99) {
                    return x * 10000 + y;
                }
            }
            else {
                if found_left { break 'x_loop; }
            }
        }
        found_left = false;
    }
    -6969
}

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let mut helper = aoc_helper::Helper::new_with_serializer(2019, 19, day19_gen);
    helper.part1(part1);
    helper.part2(part2);
    helper.run()?;
    Ok(())
}
