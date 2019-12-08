const POW10: [i32; 6] = [1, 10, 100, 1000, 10000, 100000];

/// Returns the `n`th digit (from right to left) of the given six-digit `num`.
fn nth_digit(num: i32, n: usize) -> i32 {
    (num / POW10[n]) % 10
}

#[derive(Debug, Copy, Clone)]
pub enum Op {
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
pub enum ParamMode {
    Position,
    Immediate,
}

impl Default for ParamMode {
    fn default() -> ParamMode {
        ParamMode::Position
    }
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

#[derive(Debug, Copy, Clone, Default)]
pub struct Param {
    value: i32,
    mode: ParamMode,
}

#[derive(Debug, Copy, Clone)]
pub struct Instruction {
    opcode: Op,
    params: [Param; 3],
    length: usize,
}

pub fn decode_instr(prog: &[i32], pc: usize) -> Instruction {
    let instr = prog[pc];
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
        _ => panic!("Illegal instruction {} at PC={}", instr, pc),
    };

    let mut params = [Param::default(); 3];
    for i in 1..length {
        params[i - 1] = Param {
            value: prog[pc + i],
            mode: ParamMode::from(nth_digit(instr, i + 1)),
        };
    }
    
    Instruction { opcode, params, length }
}

fn read_value(prog: &[i32], Param { value, mode }: Param) -> i32 {
    match mode {
        ParamMode::Position => prog[value as usize],
        ParamMode::Immediate => value,
    }
}

#[derive(Debug, Copy, Clone)]
pub enum IOOperation {
    Input,
    Output(i32),
}

#[derive(Debug, Copy, Clone, Default)]
pub struct RunResult {
    pub last_pc: usize,
    pub halted: bool,
}

/// Runs the given Intcode program using the provided program counter and I/O handler.
///
/// The `io_handler` closure should return either the input value, or a `-1` on output if execution should pause, causing the function to return.
///
/// Returns the status of execution.
pub fn run<F>(prog: &mut [i32], pc: usize, io_handler: F) -> RunResult
where
    F: FnMut(IOOperation) -> i32
{
    let mut pc = pc; // Program counter
    let mut io_handler = io_handler;
    let mut halted = false;

    while !halted {
        let mut pc_increase = true;
        let ins = decode_instr(prog, pc);
        match ins.opcode {
            Op::Add => {
                let left_operand = read_value(prog, ins.params[0]);
                let right_operand = read_value(prog, ins.params[1]);
                let write_idx = ins.params[2].value as usize;
                prog[write_idx] = left_operand + right_operand;
            },
            Op::Multiply => {
                let left_operand = read_value(prog, ins.params[0]);
                let right_operand = read_value(prog, ins.params[1]);
                let write_idx = ins.params[2].value as usize;
                prog[write_idx] = left_operand * right_operand;
            },
            Op::Input => {
                let write_idx = ins.params[0].value as usize;
                prog[write_idx] = io_handler(IOOperation::Input);
            },
            Op::Output => {
                let value = read_value(prog, ins.params[0]);
                let continue_code = io_handler(IOOperation::Output(value));
                if continue_code == -1 { // Pause execution
                    pc += ins.length;
                    break;
                }
            },
            Op::JumpIfTrue => {
                let value = read_value(prog, ins.params[0]);
                let dest = read_value(prog, ins.params[1]);
                if value != 0 {
                    pc = dest as usize;
                    pc_increase = false;
                }
            },
            Op::JumpIfFalse => {
                let value = read_value(prog, ins.params[0]);
                let dest = read_value(prog, ins.params[1]);
                if value == 0 {
                    pc = dest as usize;
                    pc_increase = false;
                }
            },
            Op::LessThan => {
                let left_operand = read_value(prog, ins.params[0]);
                let right_operand = read_value(prog, ins.params[1]);
                let write_idx = ins.params[2].value as usize;
                prog[write_idx] = (left_operand < right_operand) as i32;
            },
            Op::Equals => {
                let left_operand = read_value(prog, ins.params[0]);
                let right_operand = read_value(prog, ins.params[1]);
                let write_idx = ins.params[2].value as usize;
                prog[write_idx] = (left_operand == right_operand) as i32;
            },
            Op::Halt => {
                halted = true;
                pc_increase = false;
            },
        }

        if pc_increase {
            pc += ins.length;
        }
    }
    
    RunResult {
        last_pc: pc,
        halted,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::day5;
    use crate::day7;

    fn read_intcode_input(path: &str) -> Vec<i32> {
        let input = std::fs::read_to_string(path).unwrap();
        input
            .split(',')
            .flat_map(|num_str| num_str.trim().parse::<i32>())
            .collect()
    }

    #[test]
    fn day2_part1() {
        let mut prog = read_intcode_input("input/2019/day2.txt");
        prog[1] = 12;
        prog[2] = 2;
        run(&mut prog, 0, |_| { 0 });
        assert_eq!(prog[0], 6327510);
    }

    #[test]
    fn day5_part1() {
        let prog = read_intcode_input("input/2019/day5.txt");
        assert_eq!(day5::part1(&prog), 16434972);
    }

    #[test]
    fn day5_part2() {
        let prog = read_intcode_input("input/2019/day5.txt");
        assert_eq!(day5::part2(&prog), 16694270);
    }

    #[test]
    fn day7_part1() {
        let prog = read_intcode_input("input/2019/day7.txt");
        assert_eq!(day7::part1(&prog), 359142);
    }

    #[test]
    fn day7_part2() {
        let prog = read_intcode_input("input/2019/day7.txt");
        assert_eq!(day7::part2(&prog), 4374895);
    }
}
