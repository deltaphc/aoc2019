use aoc_runner_derive::{aoc, aoc_generator};
use crate::intcode::{self, IOOperation, RunResult};
use itertools::Itertools;

#[aoc_generator(day7)]
fn day7_gen(input: &str) -> Vec<i32> {
    input
        .split(',')
        .flat_map(|num_str| num_str.trim().parse::<i32>())
        .collect()
}

fn run_amplifier(prog: &mut [i32], pc: usize, phase_setting: impl Into<Option<i32>>, input_signal: i32) -> (i32, RunResult) {
    let mut output = 0;
    let phase_setting = phase_setting.into();
    let mut input_iter = std::iter::once(phase_setting.unwrap_or(0)).chain(std::iter::repeat(input_signal));
    if phase_setting.is_none() {
        input_iter.next();
    }
    let run_result = intcode::run(prog, pc, |io_op| {
        match io_op {
            IOOperation::Input => input_iter.next().unwrap(),
            IOOperation::Output(value) => {
                output = value;
                return -1; //Tells intcode to pause execution and break immediately
            },
        }
    });
    (output, run_result)
}

#[aoc(day7, part1)]
pub(crate) fn part1(input: &[i32]) -> i32 {
    let mut highest_output = 0;
    for phases in (0..=4).permutations(5) {
        let [a, b, c, d, e] = [phases[0], phases[1], phases[2], phases[3], phases[4]];
        let mut output;
        output = run_amplifier(&mut input.to_vec(), 0, a, 0);
        output = run_amplifier(&mut input.to_vec(), 0, b, output.0);
        output = run_amplifier(&mut input.to_vec(), 0, c, output.0);
        output = run_amplifier(&mut input.to_vec(), 0, d, output.0);
        output = run_amplifier(&mut input.to_vec(), 0, e, output.0);
        if output.0 > highest_output {
            highest_output = output.0;
        }
    }
    highest_output
}

#[aoc(day7, part2)]
pub(crate) fn part2(input: &[i32]) -> i32 {
    let mut highest_output = 0;
    for phases in (5..=9).permutations(5) {
        let mut output = (0, RunResult::default());
        let mut amp_progs = [input.to_vec(), input.to_vec(), input.to_vec(), input.to_vec(), input.to_vec()];
        let mut amp_pc = [0; 5];
        let mut provide_phase = true;
        while !output.1.halted {
            if provide_phase {
                for i in 0..5 {
                    output = run_amplifier(&mut amp_progs[i], amp_pc[i], phases[i], output.0);
                    amp_pc[i] = output.1.last_pc;
                }
                provide_phase = false;
            }
            else {
                for i in 0..5 {
                    output = run_amplifier(&mut amp_progs[i], amp_pc[i], None, output.0);
                    amp_pc[i] = output.1.last_pc;
                }
            }

            if output.0 > highest_output {
                highest_output = output.0;
            }
        }
    }
    highest_output
}
