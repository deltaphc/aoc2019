use aoc_runner_derive::{aoc, aoc_generator};
use crate::{intcode, intcode::IOOperation};
use itertools::Itertools;

#[aoc_generator(day7)]
fn day7_gen(input: &str) -> Vec<i32> {
    input
        .split(',')
        .flat_map(|num_str| num_str.parse::<i32>())
        .collect()
}

fn run_amplifier(prog: &mut [i32], phase_setting: i32, input_signal: i32) -> i32 {
    let mut output = 0;
    let mut input_iter = std::iter::once(phase_setting).chain(std::iter::once(input_signal));
    intcode::run(prog, |io_op| {
        match io_op {
            IOOperation::Input => input_iter.next().unwrap(),
            IOOperation::Output(value) => { output = value; return 0; },
        }
    });
    output
}

#[aoc(day7, part1)]
fn part1(input: &[i32]) -> i32 {
    let mut highest_output = 0;
    for phases in (0..=4).permutations(5) {
        let [a, b, c, d, e] = [phases[0], phases[1], phases[2], phases[3], phases[4]];
        let mut output;
        output = run_amplifier(&mut input.to_vec(), a, 0);
        output = run_amplifier(&mut input.to_vec(), b, output);
        output = run_amplifier(&mut input.to_vec(), c, output);
        output = run_amplifier(&mut input.to_vec(), d, output);
        output = run_amplifier(&mut input.to_vec(), e, output);
        if output > highest_output {
            highest_output = output;
        }
    }
    highest_output
}
