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
pub struct Instruction {
    opcode: Op,
    param_modes: [ParamMode; 3],
    params: [i32; 3],
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

    let mut params = [0_i32; 3];
    for i in 1..length {
        params[i - 1] = prog[pc + i];
    }

    let param_modes = [
        ParamMode::from(nth_digit(instr, 2)),
        ParamMode::from(nth_digit(instr, 3)),
        ParamMode::from(nth_digit(instr, 4)),
    ];
    
    Instruction { opcode, param_modes, params, length }
}

fn read_value(prog: &[i32], num: i32, param_mode: ParamMode) -> i32 {
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
                let left_operand = read_value(prog, ins.params[0], ins.param_modes[0]);
                let right_operand = read_value(prog, ins.params[1], ins.param_modes[1]);
                prog[ins.params[2] as usize] = left_operand + right_operand;
            },
            Op::Multiply => {
                let left_operand = read_value(prog, ins.params[0], ins.param_modes[0]);
                let right_operand = read_value(prog, ins.params[1], ins.param_modes[1]);
                prog[ins.params[2] as usize] = left_operand * right_operand;
            },
            Op::Input => {
                prog[ins.params[0] as usize] = io_handler(IOOperation::Input);
            },
            Op::Output => {
                let value = read_value(prog, ins.params[0], ins.param_modes[0]);
                io_handler(IOOperation::Output(value));
            },
            Op::JumpIfTrue => {
                let value = read_value(prog, ins.params[0], ins.param_modes[0]);
                let dest = read_value(prog, ins.params[1], ins.param_modes[1]);
                if value != 0 {
                    pc = dest as usize;
                    pc_increase = false;
                }
            },
            Op::JumpIfFalse => {
                let value = read_value(prog, ins.params[0], ins.param_modes[0]);
                let dest = read_value(prog, ins.params[1], ins.param_modes[1]);
                if value == 0 {
                    pc = dest as usize;
                    pc_increase = false;
                }
            },
            Op::LessThan => {
                let left_operand = read_value(prog, ins.params[0], ins.param_modes[0]);
                let right_operand = read_value(prog, ins.params[1], ins.param_modes[1]);
                prog[ins.params[2] as usize] = (left_operand < right_operand) as i32;
            },
            Op::Equals => {
                let left_operand = read_value(prog, ins.params[0], ins.param_modes[0]);
                let right_operand = read_value(prog, ins.params[1], ins.param_modes[1]);
                prog[ins.params[2] as usize] = (left_operand == right_operand) as i32;
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

    #[test]
    fn day2_input() {
        const INPUT: &str = include_str!("../input/2019/day2.txt");
        let mut prog: Vec<i32> = INPUT
            .trim()
            .split(',')
            .flat_map(|num_str| num_str.parse::<i32>())
            .collect();
        
        prog[1] = 12;
        prog[2] = 2;

        run(&mut prog, |_| { 0 });

        assert_eq!(prog[0], 6327510);
    }

    #[test]
    fn day5_input() {
        const INPUT: &str = include_str!("../input/2019/day5.txt");
        let default_prog = INPUT
            .trim()
            .split(',')
            .flat_map(|num_str| num_str.parse::<i32>())
            .collect::<Vec<i32>>()
            .into_boxed_slice();
        
        {
            let mut prog = default_prog.to_vec();
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
        {
            let mut prog = default_prog.to_vec();
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
}
