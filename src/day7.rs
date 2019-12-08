use aoc_runner_derive::{aoc, aoc_generator};
use crate::intcode::{Program, IOOperation};
use itertools::Itertools;

#[aoc_generator(day7)]
fn day7_gen(input: &str) -> Vec<i32> {
    input
        .split(',')
        .flat_map(|num_str| num_str.trim().parse::<i32>())
        .collect()
}

#[derive(Debug, Copy, Clone, Default)]
struct AmpResult {
    output: i32,
    halted: bool,
}

fn run_amplifier(prog: &mut Program, phase_setting: impl Into<Option<i32>>, input_signal: i32) -> AmpResult {
    let mut output = 0;
    let phase_setting = phase_setting.into();
    let mut input_iter = std::iter::once(phase_setting.unwrap_or(0)).chain(std::iter::repeat(input_signal));
    if phase_setting.is_none() {
        input_iter.next();
    }
    let halted = prog.run(|io_op| {
        match io_op {
            IOOperation::Input => input_iter.next().unwrap(),
            IOOperation::Output(value) => {
                output = value;
                return -1; //Tells intcode to pause execution and break immediately
            },
        }
    });
    AmpResult { output, halted }
}

#[aoc(day7, part1)]
pub(crate) fn part1(input: &[i32]) -> i32 {
    let mut highest_output = 0;
    for phases in (0..=4).permutations(5) {
        let mut amp_progs = [Program::from(input), Program::from(input), Program::from(input), Program::from(input), Program::from(input)];
        let mut amp_result = AmpResult::default();
        for i in 0..5 {
           amp_result = run_amplifier(&mut amp_progs[i], phases[i], amp_result.output);
        }
        if amp_result.output > highest_output {
            highest_output = amp_result.output;
        }
    }
    highest_output
}

#[aoc(day7, part2)]
pub(crate) fn part2(input: &[i32]) -> i32 {
    let mut highest_output = 0;
    for phases in (5..=9).permutations(5) {
        let mut amp_progs = [Program::from(input), Program::from(input), Program::from(input), Program::from(input), Program::from(input)];
        let mut amp_result = AmpResult::default();
        let mut provide_phase = true;
        while !amp_result.halted {
            if provide_phase {
                for i in 0..5 {
                    amp_result = run_amplifier(&mut amp_progs[i], phases[i], amp_result.output);
                }
                provide_phase = false;
            }
            else {
                for i in 0..5 {
                    amp_result = run_amplifier(&mut amp_progs[i], None, amp_result.output);
                }
            }

            if amp_result.output > highest_output {
                highest_output = amp_result.output;
            }
        }
    }
    highest_output
}
