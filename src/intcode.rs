use rustc_hash::FxHashMap;
use std::collections::VecDeque;

// Type for the integers used by the computer.
pub type Int = i128;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum RunResult {
    Output(Int),
    Finished,
}

#[derive(Default, Clone)]
pub struct IntcodeComputer {
    memory: FxHashMap<Int, Int>,
    input_queue: VecDeque<Int>,
    ip: Int,
    rel_base: Int,
    is_finished: bool,
}

//////////////////////////////////////////////////////////////////////////////////////////////////////
/// Internal stuff

// Intcode operation codes.
struct Opcodes;
impl Opcodes {
    pub const ADD: u8 = 1;
    pub const MUL: u8 = 2;
    pub const IN:  u8 = 3;
    pub const OUT: u8 = 4;
    pub const JMP: u8 = 5;
    pub const JMN: u8 = 6;
    pub const LT:  u8 = 7;
    pub const EQ:  u8 = 8;
    pub const RLB: u8 = 9;
    pub const END: u8 = 99;
}

// Parameter modes
#[derive(Default, Copy, Clone)]
enum ParamMode {
    #[default]
    Position,
    Immediate,
    Relative,
}

#[derive(Default, Copy, Clone)]
struct Param {
    mode: ParamMode,
    value: Int,
}

// These intcode computers are one-time use only, proudly contributing to e-waste.
impl IntcodeComputer {

    pub fn new(code: &[Int]) -> Self {
        let memory = code.iter().enumerate().map(|(i, v)| (i as Int, *v)).collect();
        Self { memory, input_queue: VecDeque::new(), ip: 0, rel_base: 0, is_finished: false }
    }

    pub fn input(&mut self, value: Int) {
        self.input_queue.push_back(value);
    }

    pub fn run(&mut self) -> RunResult {
        while !self.is_finished {
            let (opcode, params) = self.parse_operation();

            match opcode {
                Opcodes::ADD => self.op_add(&params),
                Opcodes::MUL => self.op_mul(&params),
                Opcodes::IN => self.op_in(&params),
                Opcodes::OUT => {
                    let ret = self.param_value(&params[0]);
                    return RunResult::Output(ret);
                },
                Opcodes::JMP => self.op_jmp(&params),
                Opcodes::JMN => self.op_jmn(&params),
                Opcodes::LT => self.op_lt(&params),
                Opcodes::EQ => self.op_eq(&params),
                Opcodes::RLB => self.op_rlb(&params),
                Opcodes::END => self.is_finished = true,
                x => panic!("Unexpected opcode: {x}"),
            }
        }

        RunResult::Finished
    }

    pub fn read_at(&self, pos: Int) -> Int {
        // Reads raw data from memory from a given position
        self.memory.get(&pos).copied().unwrap_or_default()
    }

    //////////////////////////////////////////////////////////////////////////////////////////////////////

    fn op_add(&mut self, params: &[Param]) {
        let v1 = self.param_value(&params[0]);
        let v2 = self.param_value(&params[1]);
        let res = v1 + v2;
        self.write_to(&params[2], res);
    }

    fn op_mul(&mut self, params: &[Param]) {
        let v1 = self.param_value(&params[0]);
        let v2 = self.param_value(&params[1]);
        let res = v1 * v2;
        self.write_to(&params[2], res);
    }

    fn op_in(&mut self, params: &[Param]) {
        // I just wanted to implement the thing, not solve the rest of the problems
        // that involve using the computer, so I made the simplification of assuming
        // that an input will always be available.
        let input = self.input_queue.pop_front().expect("No input available");
        self.write_to(&params[0], input);
    }

    fn op_jmp(&mut self, params: &[Param]) {
        let val = self.param_value(&params[0]);
        if val != 0 {
            self.ip = self.param_value(&params[1]);
        }
    }

    fn op_jmn(&mut self, params: &[Param]) {
        let val = self.param_value(&params[0]);
        if val == 0 {
            self.ip = self.param_value(&params[1]);
        }
    }

    fn op_lt(&mut self, params: &[Param]) {
        let v1 = self.param_value(&params[0]);
        let v2 = self.param_value(&params[1]);
        let res = (v1 < v2) as Int;
        self.write_to(&params[2], res);
    }

    fn op_eq(&mut self, params: &[Param]) {
        let v1 = self.param_value(&params[0]);
        let v2 = self.param_value(&params[1]);
        let res = (v1 == v2) as Int;
        self.write_to(&params[2], res);
    }

    fn op_rlb(&mut self, params: &[Param]) {
        let val = self.param_value(&params[0]);
        self.rel_base += val;
    }

    //////////////////////////////////////////////////////////////////////////////////////////////////////

    fn parse_operation(&mut self) -> (u8, [Param; 3]) {
        let opcode = (self.memory[&self.ip] % 100) as u8;
        let mut flags = self.memory[&self.ip] / 100;
        let n_params = match opcode {
            Opcodes::END                                            => 0,
            Opcodes::IN  | Opcodes::OUT | Opcodes::RLB              => 1,
            Opcodes::JMP | Opcodes::JMN                             => 2,
            Opcodes::ADD | Opcodes::MUL | Opcodes::EQ | Opcodes::LT => 3,
            _ => panic!("Unknown opcode"),
        };
        let mut params = [Param::default(); 3];

        for i in 0..n_params {
            let mode = match flags % 10 {
                0 => ParamMode::Position,
                1 => ParamMode::Immediate,
                2 => ParamMode::Relative,
                x => panic!("Unknown param mode: {x}"),
            };
            flags /= 10;
            let value = self.read_at(self.ip + i as Int + 1);
            params[i] = Param{ mode, value };
        }
        self.ip += 1 + n_params as Int;
        (opcode, params)
    }

    fn param_value(&self, param: &Param) -> Int {
        match param.mode {
            ParamMode::Immediate => param.value,
            ParamMode::Position => self.read_at(param.value),
            ParamMode::Relative => self.read_at(param.value + self.rel_base),
        }
    }

    fn write_to(&mut self, param: &Param, value: Int) {
        let addr = match param.mode {
            ParamMode::Immediate => panic!("Output addresses cannot be in immediate mode"),
            ParamMode::Position => param.value,
            ParamMode::Relative => param.value + self.rel_base,
        };
        self.memory.insert(addr, value);
    }
}

impl<T: AsRef<str>> From<T> for IntcodeComputer {
    fn from(code: T) -> Self {
        let vec: Vec<Int> = code.as_ref().trim().split(',').map(|x| x.trim().parse().unwrap()).collect();
        Self::new(&vec)
    }
}