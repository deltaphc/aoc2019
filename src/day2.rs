use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day2)]
fn day2_gen(input: &str) -> Vec<usize> {
    input
        .split(',')
        .flat_map(|num| num.parse::<usize>().ok())
        .collect()
}

fn run_program(program: &mut [usize], input1: usize, input2: usize) -> usize {
    program[1] = input1;
    program[2] = input2;

    for i in (0..program.len()).step_by(4) {
        match program[i] {
            1 => {
                let i1 = program[i + 1];
                let i2 = program[i + 2];
                let i3 = program[i + 3];
                program[i3] = program[i1] + program[i2];
            },
            2 => {
                let i1 = program[i + 1];
                let i2 = program[i + 2];
                let i3 = program[i + 3];
                program[i3] = program[i1] * program[i2];
            },
            99 => break,
            _ => panic!("Invalid opcode at position {}!", i),
        }
    }

    program[0]
}

#[aoc(day2, part1)]
fn part1(input: &[usize]) -> usize {
    let mut program = input.to_vec();
    run_program(&mut program, 12, 2)
}

#[aoc(day2, part2)]
fn part2(input: &[usize]) -> usize {
    let mut program = vec![0; input.len()];
    for verb in 0..=99 {
        for noun in 0..=99 {
            program.copy_from_slice(input);
            let num = run_program(&mut program, noun, verb);
            if num != 19690720 { continue; }
            return 100 * noun + verb;
        }
    }

    panic!("No pair was found.")
}
