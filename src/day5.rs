use aoc_runner_derive::{aoc, aoc_generator};

const POW10: [i32; 6] = [1, 10, 100, 1000, 10000, 100000];

/// Returns the `n`th digit (from right to left) of the given six-digit `num`.
fn nth_digit(num: i32, n: usize) -> i32 {
    (num / POW10[n]) % 10
}

#[derive(Debug, Copy, Clone)]
enum Op {
    Add,
    Multiply,
    Input,
    Output,
    JumpIfTrue,
    JumpIfFalse,
    LessThan,
    Equals,
    Halt,
}

#[derive(Debug, Copy, Clone)]
enum ParamMode {
    Position,
    Immediate,
}

impl From<i32> for ParamMode {
    fn from(num: i32) -> ParamMode {
        match num {
            0 => ParamMode::Position,
            1 => ParamMode::Immediate,
            _ => panic!("Invalid parameter mode {}", num),
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Instruction {
    opcode: Op,
    param_modes: [ParamMode; 3],
    length: usize,
}

fn decode_instr(instr: i32) -> Instruction {
    let (opcode, length) = match instr % 100 { // first two digits
        1 => (Op::Add, 4),
        2 => (Op::Multiply, 4),
        3 => (Op::Input, 2),
        4 => (Op::Output, 2),
        5 => (Op::JumpIfTrue, 3),
        6 => (Op::JumpIfFalse, 3),
        7 => (Op::LessThan, 4),
        8 => (Op::Equals, 4),
        99 => (Op::Halt, 1),
        _ => panic!("Illegal instruction {}", instr),
    };

    let param_modes = [
        ParamMode::from(nth_digit(instr, 2)),
        ParamMode::from(nth_digit(instr, 3)),
        ParamMode::from(nth_digit(instr, 4)),
    ];
    
    Instruction { opcode, param_modes, length }
}

fn read_value(prog: &[i32], num: i32, param_mode: ParamMode) -> i32 {
    match param_mode {
        ParamMode::Position => prog[num as usize],
        ParamMode::Immediate => num,
    }
}

fn run_intcode(prog: &mut [i32]) {
    let mut pc = 0_usize; // Program counter
    let mut halted = false;

    while !halted {
        let mut pc_increase = true;
        let instruction = decode_instr(prog[pc]);
        match instruction.opcode {
            Op::Add => {
                let i1 = prog[pc + 1];
                let i2 = prog[pc + 2];
                let write_idx = prog[pc + 3] as usize;
                prog[write_idx] =
                    read_value(prog, i1, instruction.param_modes[0]) +
                    read_value(prog, i2, instruction.param_modes[1]);
            },
            Op::Multiply => {
                let i1 = prog[pc + 1];
                let i2 = prog[pc + 2];
                let write_idx = prog[pc + 3] as usize;
                prog[write_idx] =
                    read_value(prog, i1, instruction.param_modes[0]) *
                    read_value(prog, i2, instruction.param_modes[1]);
            },
            Op::Input => {
                let mut input_buf = String::new();
                std::io::stdin().read_line(&mut input_buf)
                    .expect("Intcode input error");
                
                let write_idx = prog[pc + 1] as usize;
                prog[write_idx] = input_buf.trim().parse::<i32>()
                    .expect("Invalid user input");
            },
            Op::Output => {
                let i1 = prog[pc + 1];
                let value = read_value(prog, i1, instruction.param_modes[0]);
                println!("{}", value);
            },
            Op::JumpIfTrue => {
                let i1 = prog[pc + 1];
                let i2 = prog[pc + 2];
                let value = read_value(prog, i1, instruction.param_modes[0]);
                let dest = read_value(prog, i2, instruction.param_modes[1]);
                if value != 0 {
                    pc = dest as usize;
                    pc_increase = false;
                }
            },
            Op::JumpIfFalse => {
                let i1 = prog[pc + 1];
                let i2 = prog[pc + 2];
                let value = read_value(prog, i1, instruction.param_modes[0]);
                let dest = read_value(prog, i2, instruction.param_modes[1]);
                if value == 0 {
                    pc = dest as usize;
                    pc_increase = false;
                }
            },
            Op::LessThan => {
                let i1 = prog[pc + 1];
                let i2 = prog[pc + 2];
                let write_idx = prog[pc + 3] as usize;
                prog[write_idx] = (
                    read_value(prog, i1, instruction.param_modes[0]) <
                    read_value(prog, i2, instruction.param_modes[1])) as i32;
            },
            Op::Equals => {
                let i1 = prog[pc + 1];
                let i2 = prog[pc + 2];
                let write_idx = prog[pc + 3] as usize;
                prog[write_idx] = (
                    read_value(prog, i1, instruction.param_modes[0]) ==
                    read_value(prog, i2, instruction.param_modes[1])) as i32;
            },
            Op::Halt => halted = true,
        }

        if pc_increase {
            pc += instruction.length;
        }
    }
}

#[aoc_generator(day5)]
fn day5_gen(input: &str) -> Vec<i32> {
    input
        .split(',')
        .flat_map(|num_str| num_str.parse::<i32>())
        .collect()
}

#[aoc(day5, part1)]
fn part1(input: &[i32]) -> i32 {
    let mut prog = input.to_vec();
    run_intcode(&mut prog);
    -6969 //garbage value to satisfy cargo-aoc
}

#[aoc(day5, part2)]
fn part2(input: &[i32]) -> i32 {
    let mut prog = input.to_vec();
    run_intcode(&mut prog);
    -6969 //garbage value to satisfy cargo-aoc
}
