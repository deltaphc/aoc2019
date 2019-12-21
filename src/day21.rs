// WIP

use crate::intcode::prelude::*;

fn day21_gen(input: String) -> Vec<i64> {
    input
        .split(',')
        .flat_map(|num_str| num_str.trim().parse::<i64>())
        .collect()
}

const SPRINGSCRIPT: &[u8] =
b"NOT A J
NOT B T
AND T J
NOT C T
AND T J
AND D J
WALK
";

fn part1(input: Vec<i64>) -> i64 {
    let mut script_iter = SPRINGSCRIPT.iter().copied();
    let mut prog = Program::from(input.as_slice());
    let mut hull_damage = -1;
    prog.run(|io_op| {
        match io_op {
            IOOperation::Input => IOReturn::Input(script_iter.next().unwrap() as i64),
            IOOperation::Output(value) => {
                if value < 128 { // inside ASCII range?
                    print!("{}", char::from(value as u8));
                    IOReturn::Output(ExecuteAction::Continue)
                }
                else {
                    hull_damage = value;
                    IOReturn::Output(ExecuteAction::Break)
                }
            }
        }
    });
    hull_damage
}

fn part2(input: Vec<i64>) -> i64 {
    -6969
}

use aoc_helper::{AocDay, Puzzle};
pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let mut day = AocDay::new_with_serializer(2019, 21, day21_gen);
    let part1 = Puzzle::new(1, part1);
    let part2 = Puzzle::new(2, part2);
    day.run(&part1)?;
    day.run(&part2)?;
    Ok(())
}
