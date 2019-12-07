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
pub struct Param(i32, ParamMode);

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

    let mut params = [Param(0, ParamMode::Immediate); 3];
    for i in 1..length {
        params[i - 1] = Param(
            prog[pc + i],
            ParamMode::from(nth_digit(instr, i + 1))
        );
    }
    
    Instruction { opcode, params, length }
}

fn read_value(prog: &[i32], Param(num, param_mode): Param) -> i32 {
    match param_mode {
        ParamMode::Position => prog[num as usize],
        ParamMode::Immediate => num,
    }
}

#[derive(Debug, Copy, Clone)]
pub enum IOOperation {
    Input,
    Output(i32),
}

pub fn run<F>(prog: &mut [i32], mut io_handler: F)
where
    F: FnMut(IOOperation) -> i32
{
    let mut pc = 0_usize; // Program counter
    let mut halted = false;

    while !halted {
        let mut pc_increase = true;
        let ins = decode_instr(prog, pc);
        match ins.opcode {
            Op::Add => {
                let left_operand = read_value(prog, ins.params[0]);
                let right_operand = read_value(prog, ins.params[1]);
                let write_idx = ins.params[2].0 as usize;
                prog[write_idx] = left_operand + right_operand;
            },
            Op::Multiply => {
                let left_operand = read_value(prog, ins.params[0]);
                let right_operand = read_value(prog, ins.params[1]);
                let write_idx = ins.params[2].0 as usize;
                prog[write_idx] = left_operand * right_operand;
            },
            Op::Input => {
                let write_idx = ins.params[0].0 as usize;
                prog[write_idx] = io_handler(IOOperation::Input);
            },
            Op::Output => {
                let value = read_value(prog, ins.params[0]);
                io_handler(IOOperation::Output(value));
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
                let write_idx = ins.params[2].0 as usize;
                prog[write_idx] = (left_operand < right_operand) as i32;
            },
            Op::Equals => {
                let left_operand = read_value(prog, ins.params[0]);
                let right_operand = read_value(prog, ins.params[1]);
                let write_idx = ins.params[2].0 as usize;
                prog[write_idx] = (left_operand == right_operand) as i32;
            },
            Op::Halt => halted = true,
        }

        if pc_increase {
            pc += ins.length;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn read_intcode_input(path: &str) -> Vec<i32> {
        let input = std::fs::read_to_string(path).unwrap();
        input
            .trim()
            .split(',')
            .flat_map(|num_str| num_str.parse::<i32>())
            .collect()
    }

    #[test]
    fn day2_part1() {
        let mut prog = read_intcode_input("input/2019/day2.txt");
        prog[1] = 12;
        prog[2] = 2;
        run(&mut prog, |_| { 0 });
        assert_eq!(prog[0], 6327510);
    }

    #[test]
    fn day5_part1() {
        let mut prog = read_intcode_input("input/2019/day5.txt");
        let mut output = -6969;
        run(&mut prog, |io_op| {
            match io_op {
                IOOperation::Input => return 1,
                IOOperation::Output(value) => {
                    output = value;
                    return 0;
                },
            }
        });
        assert_eq!(output, 16434972);
    }

    #[test]
    fn day5_part2() {
        let mut prog = read_intcode_input("input/2019/day5.txt");
        let mut output = -6969;
        run(&mut prog, |io_op| {
            match io_op {
                IOOperation::Input => return 5,
                IOOperation::Output(value) => {
                    output = value;
                    return 0;
                },
            }
        });
        assert_eq!(output, 16694270);
    }
}
