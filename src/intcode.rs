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

impl From<u8> for ParamMode {
    fn from(num: u8) -> ParamMode {
        match num {
            0 => ParamMode::Position,
            1 => ParamMode::Immediate,
            2 => ParamMode::Relative,
            _ => panic!("Invalid parameter mode {}", num),
        }
    }
}

#[derive(Debug, Copy, Clone)]
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

#[derive(Debug, Copy, Clone)]
pub enum IOReturn {
    Input(i64),
    Output(ExecuteAction),
}

impl IOReturn {
    pub fn input_value(self) -> i64 {
        match self {
            IOReturn::Input(value) => value,
            IOReturn::Output(_) => panic!("Attempted to get input value from IOReturn::Output"),
        }
    }

    pub fn exec_action(self) -> ExecuteAction {
        match self {
            IOReturn::Input(_) => panic!("Attempted to get execution action from IOReturn::Input"),
            IOReturn::Output(exec_action) => exec_action,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum ExecuteAction {
    Continue,
    Break,
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
    #[allow(dead_code)]
    pub fn prog(&self) -> &[i64] {
        &self.prog
    }

    #[allow(dead_code)]
    pub fn prog_mut(&mut self) -> &mut [i64] {
        &mut self.prog
    }

    pub fn is_halted(&self) -> bool {
        self.halted
    }

    fn decode(&self) -> Instruction {
        let mut instr = unsafe { *self.prog.get_unchecked(self.pc) } as u16; //gives slightly better perf on div/mod than i64
        let op = instr % 100;
        instr /= 100;
        let mode0 = instr % 10;
        instr /= 10;
        let mode1 = instr % 10;
        instr /= 10;
        let mode2 = instr % 10;

        let (length, opcode) = match op {
            1 => (4, Op::Add),
            2 => (4, Op::Multiply),
            3 => (2, Op::Input),
            4 => (2, Op::Output),
            5 => (3, Op::JumpIfTrue),
            6 => (3, Op::JumpIfFalse),
            7 => (4, Op::LessThan),
            8 => (4, Op::Equals),
            9 => (2, Op::RelativeBase),
            99 => (1, Op::Halt),
            _ => panic!("Illegal instruction {} at PC={}", self.prog[self.pc], self.pc),
        };

        let params = [
            Param {
                value: unsafe { *self.prog.get_unchecked(self.pc + 1) },
                mode: ParamMode::from(mode0 as u8),
            },
            Param {
                value: unsafe { *self.prog.get_unchecked(self.pc + 2) },
                mode: ParamMode::from(mode1 as u8),
            },
            Param {
                value: unsafe { *self.prog.get_unchecked(self.pc + 3) },
                mode: ParamMode::from(mode2 as u8),
            },
        ];
        
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
                self.prog.extend(std::iter::repeat(0).take(extend_len + 3)); // 3-padding to guarantee being able to read 3 values for opcode decoding
            }
        }

        match param.mode {
            // We can use `get_unchecked` here because by casting to usize, we know we're not negative.
            // We've also verified that the program is large enough to contain the index.
            ParamMode::Position | ParamMode::Relative => unsafe { *self.prog.get_unchecked(read_idx) },
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
            self.prog.extend(std::iter::repeat(0).take(extend_len + 3)); // 3-padding to guarantee being able to read 3 values for opcode decoding
        }

        // We can use `get_unchecked_mut` here because by casting to usize, we know we're not negative.
        // We've also verified that the program is large enough to contain the index.
        unsafe { *self.prog.get_unchecked_mut(write_idx) = write_value; }
    }

    /// Executes the given decoded instruction, and returns whether the execution loop should pause early.
    fn execute<F>(&mut self, ins: Instruction, io_handler: &mut F) -> ExecuteAction
    where
        F: FnMut(IOOperation) -> IOReturn
    {
        let mut exec_action = ExecuteAction::Continue;
        let mut pc_increase = true;

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
                self.write_value(ins.params[0], io_handler(IOOperation::Input).input_value());
            },
            Op::Output => {
                let value = self.read_value(ins.params[0]);
                exec_action = io_handler(IOOperation::Output(value)).exec_action();
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

        exec_action
    }

    /// Runs the current Intcode program using the provided I/O handler.
    pub fn run<F>(&mut self, io_handler: F)
    where
        F: FnMut(IOOperation) -> IOReturn
    {
        let mut io_handler = io_handler;
        while !self.halted {
            let instruction = self.decode();
            let exec_action = self.execute(instruction, &mut io_handler);
            if let ExecuteAction::Break = exec_action {
                break;
            }
        }
    }
 
    /// Resets the current Intcode program to its initial state.
    #[allow(dead_code)]
    pub fn reset(&mut self) {
        self.prog.clear();
        self.prog.extend_from_slice(&self.default_prog);
        self.prog.extend(std::iter::repeat(0).take(3)); // Add tiny padding on the end to guarantee that opcode decoding can read all values
        self.pc = 0;
        self.relative_base = 0;
        self.halted = false;
    }
}

impl From<&[i64]> for Program {
    fn from(prog: &[i64]) -> Program {
        Program {
            default_prog: prog.to_vec().into_boxed_slice(),
            prog: {
                let mut prog_vec = prog.to_vec();
                prog_vec.extend(std::iter::repeat(0).take(3));
                prog_vec
            },
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
        prog.run(|_| { IOReturn::Input(0) });
        assert_eq!(prog.prog()[0], 6327510);
    }

    #[test]
    fn day5_part1() {
        let prog = read_intcode_input("input/2019/day5.txt");
        assert_eq!(day5::part1(prog), 16434972);
    }

    #[test]
    fn day5_part2() {
        let prog = read_intcode_input("input/2019/day5.txt");
        assert_eq!(day5::part2(prog), 16694270);
    }

    #[test]
    fn day7_part1() {
        let prog = read_intcode_input("input/2019/day7.txt");
        assert_eq!(day7::part1(prog), 359142);
    }

    #[test]
    fn day7_part2() {
        let prog = read_intcode_input("input/2019/day7.txt");
        assert_eq!(day7::part2(prog), 4374895);
    }

    #[test]
    fn day9_part1() {
        let prog = read_intcode_input("input/2019/day9.txt");
        assert_eq!(day9::part1(prog), 3235019597);
    }

    #[test]
    fn day9_part2() {
        let prog = read_intcode_input("input/2019/day9.txt");
        assert_eq!(day9::part2(prog), 80274);
    }
}
