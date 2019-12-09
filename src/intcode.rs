const POW10: [i64; 6] = [1, 10, 100, 1000, 10000, 100000];

/// Returns the `n`th digit (from right to left) of the given six-digit `num`.
fn nth_digit(num: i64, n: usize) -> i64 {
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
    RelativeBase,
    Halt,
}

#[derive(Debug, Copy, Clone)]
pub enum ParamMode {
    Position,
    Immediate,
    Relative,
}

impl Default for ParamMode {
    fn default() -> ParamMode {
        ParamMode::Position
    }
}

impl From<i64> for ParamMode {
    fn from(num: i64) -> ParamMode {
        match num {
            0 => ParamMode::Position,
            1 => ParamMode::Immediate,
            2 => ParamMode::Relative,
            _ => panic!("Invalid parameter mode {}", num),
        }
    }
}

#[derive(Debug, Copy, Clone, Default)]
pub struct Param {
    value: i64,
    mode: ParamMode,
}

#[derive(Debug, Copy, Clone)]
pub struct Instruction {
    opcode: Op,
    params: [Param; 3],
    length: usize,
}

#[derive(Debug, Copy, Clone)]
pub enum IOOperation {
    Input,
    Output(i64),
}

#[derive(Debug, Clone)]
pub struct Program {
    default_prog: Box<[i64]>,
    prog: Vec<i64>,
    pc: usize,
    relative_base: usize,
    halted: bool,
}

impl Program {
    pub fn prog(&self) -> &[i64] {
        &self.prog
    }

    pub fn prog_mut(&mut self) -> &mut [i64] {
        &mut self.prog
    }

    pub fn is_halted(&self) -> bool {
        self.halted
    }

    fn decode_instr(&self) -> Instruction {
        let instr = self.prog[self.pc];
        let (opcode, length) = match instr % 100 { // first two digits
            1 => (Op::Add, 4),
            2 => (Op::Multiply, 4),
            3 => (Op::Input, 2),
            4 => (Op::Output, 2),
            5 => (Op::JumpIfTrue, 3),
            6 => (Op::JumpIfFalse, 3),
            7 => (Op::LessThan, 4),
            8 => (Op::Equals, 4),
            9 => (Op::RelativeBase, 2),
            99 => (Op::Halt, 1),
            _ => panic!("Illegal instruction {} at PC={}", instr, self.pc),
        };
    
        let mut params = [Param::default(); 3];
        for i in 1..length {
            params[i - 1] = Param {
                value: self.prog[self.pc + i],
                mode: ParamMode::from(nth_digit(instr, i + 1)),
            };
        }
        
        Instruction { opcode, params, length }
    }
    
    fn read_value(&mut self, param: Param) -> i64 {
        let read_idx = match param.mode {
            ParamMode::Position => param.value as usize,
            ParamMode::Immediate => 0,
            ParamMode::Relative => self.relative_base.wrapping_add(param.value as usize),
        };

        if let ParamMode::Position | ParamMode::Relative = param.mode {
            if read_idx >= self.prog.len() {
                let extend_len = read_idx as usize - (self.prog.len() - 1);
                self.prog.extend(std::iter::repeat(0).take(extend_len));
            }
        }

        match param.mode {
            ParamMode::Position | ParamMode::Relative => self.prog[read_idx],
            ParamMode::Immediate => param.value,
        }
    }

    fn write_value(&mut self, param: Param, write_value: i64) {
        let write_idx = match param.mode {
            ParamMode::Position => param.value as usize,
            ParamMode::Immediate => panic!("Attempted to write to immediate value. PC={}, param={:?}", self.pc, param),
            ParamMode::Relative => self.relative_base.wrapping_add(param.value as usize),
        };
        if write_idx >= self.prog.len() {
            let extend_len = write_idx - (self.prog.len() - 1);
            self.prog.extend(std::iter::repeat(0).take(extend_len));
        }
        self.prog[write_idx] = write_value;
    }

    /// Runs the current Intcode program using the provided I/O handler.
    ///
    /// The `io_handler` closure should return either the input value, or a `-1` on output if execution should pause, causing the function to return.
    pub fn run<F>(&mut self, io_handler: F)
    where
        F: FnMut(IOOperation) -> i64
    {
        let mut io_handler = io_handler;
        
        while !self.halted {
            let mut pc_increase = true;
            let ins = self.decode_instr();
            match ins.opcode {
                Op::Add => {
                    let left_operand = self.read_value(ins.params[0]);
                    let right_operand = self.read_value(ins.params[1]);
                    self.write_value(ins.params[2], left_operand + right_operand);
                },
                Op::Multiply => {
                    let left_operand = self.read_value(ins.params[0]);
                    let right_operand = self.read_value(ins.params[1]);
                    self.write_value(ins.params[2], left_operand * right_operand);
                },
                Op::Input => {
                    self.write_value(ins.params[0], io_handler(IOOperation::Input));
                },
                Op::Output => {
                    let value = self.read_value(ins.params[0]);
                    let continue_code = io_handler(IOOperation::Output(value));
                    if continue_code == -1 { // Pause execution
                        self.pc += ins.length;
                        break;
                    }
                },
                Op::JumpIfTrue => {
                    let value = self.read_value(ins.params[0]);
                    let dest = self.read_value(ins.params[1]);
                    if value != 0 {
                        self.pc = dest as usize;
                        pc_increase = false;
                    }
                },
                Op::JumpIfFalse => {
                    let value = self.read_value(ins.params[0]);
                    let dest = self.read_value(ins.params[1]);
                    if value == 0 {
                        self.pc = dest as usize;
                        pc_increase = false;
                    }
                },
                Op::LessThan => {
                    let left_operand = self.read_value(ins.params[0]);
                    let right_operand = self.read_value(ins.params[1]);
                    self.write_value(ins.params[2], (left_operand < right_operand) as i64);
                },
                Op::Equals => {
                    let left_operand = self.read_value(ins.params[0]);
                    let right_operand = self.read_value(ins.params[1]);
                    self.write_value(ins.params[2], (left_operand == right_operand) as i64);
                },
                Op::RelativeBase => {
                    let base_offset = self.read_value(ins.params[0]);
                    self.relative_base = self.relative_base.wrapping_add(base_offset as usize);
                },
                Op::Halt => {
                    self.halted = true;
                    pc_increase = false;
                },
            }

            if pc_increase {
                self.pc += ins.length;
            }
        }
    }
 
    /// Resets the current Intcode program to its initial state.
    pub fn reset(&mut self) {
        self.prog.clear();
        self.prog.extend_from_slice(&self.default_prog);
        self.pc = 0;
        self.relative_base = 0;
        self.halted = false;
    }
}

impl From<&[i64]> for Program {
    fn from(prog: &[i64]) -> Program {
        Program {
            default_prog: prog.to_vec().into_boxed_slice(),
            prog: prog.to_vec(),
            pc: 0,
            relative_base: 0,
            halted: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::day5;
    use crate::day7;
    use crate::day9;

    fn read_intcode_input(path: &str) -> Vec<i64> {
        let input = std::fs::read_to_string(path).unwrap();
        input
            .split(',')
            .flat_map(|num_str| num_str.trim().parse::<i64>())
            .collect()
    }

    #[test]
    fn day2_part1() {
        let input = read_intcode_input("input/2019/day2.txt");
        let mut prog = Program::from(&input[..]);
        {
            let prog_content = prog.prog_mut();
            prog_content[1] = 12;
            prog_content[2] = 2;
        }
        prog.run(|_| { 0 });
        assert_eq!(prog.prog()[0], 6327510);
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

    #[test]
    fn day9_part1() {
        let prog = read_intcode_input("input/2019/day9.txt");
        assert_eq!(day9::part1(&prog), 3235019597);
    }

    #[test]
    fn day9_part2() {
        let prog = read_intcode_input("input/2019/day9.txt");
        assert_eq!(day9::part2(&prog), 80274);
    }
}
