use crate::intcode::prelude::*;
use itertools::Itertools;

fn day7_gen(input: &str) -> Vec<i64> {
    input
        .split(',')
        .flat_map(|num_str| num_str.trim().parse::<i64>())
        .collect()
}

#[derive(Debug, Copy, Clone, Default)]
struct AmpResult {
    output: i64,
    halted: bool,
}

fn run_amplifier(prog: &mut Program, phase_setting: impl Into<Option<i64>>, input_signal: i64) -> AmpResult {
    let mut output = 0;
    let phase_setting = phase_setting.into();
    let mut input_iter = std::iter::once(phase_setting.unwrap_or(0)).chain(std::iter::repeat(input_signal));
    if phase_setting.is_none() {
        input_iter.next();
    }
    prog.run(|io_op| {
        match io_op {
            IOOperation::Input => IOReturn::Input(input_iter.next().unwrap()),
            IOOperation::Output(value) => {
                output = value;
                IOReturn::Output(ExecuteAction::Break)
            },
        }
    });
    AmpResult { output, halted: prog.is_halted() }
}

pub(crate) fn part1(input: Vec<i64>) -> i64 {
    let mut highest_output = 0;
    for phases in (0..=4).permutations(5) {
        let mut amp_progs = [
            Program::from(input.as_slice()),
            Program::from(input.as_slice()),
            Program::from(input.as_slice()),
            Program::from(input.as_slice()),
            Program::from(input.as_slice()),
        ];
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

pub(crate) fn part2(input: Vec<i64>) -> i64 {
    let mut highest_output = 0;
    for phases in (5..=9).permutations(5) {
        let mut amp_progs = [
            Program::from(input.as_slice()),
            Program::from(input.as_slice()),
            Program::from(input.as_slice()),
            Program::from(input.as_slice()),
            Program::from(input.as_slice()),
        ];
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

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let mut helper = aoc_helper::Helper::new_with_serializer(2019, 7, day7_gen);
    helper.part1(part1);
    helper.part2(part2);
    helper.run()?;
    Ok(())
}
