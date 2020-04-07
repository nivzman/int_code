
mod instruction;

use instruction::Instruction;
use instruction::OpCode;
use crate::errors::ExecutionError;
use crate::executor::instruction::ParameterMode;
use crate::input::Input;
use crate::output::Output;


pub struct Executor<'a, I, O> where I: Input, O: Output {
    input: I,
    memory: Vec<i32>,
    output: &'a mut O,
    current_address: usize,
    debug: DebugInfo,
    instruction: Instruction,
}

struct DebugInfo {
    raw_executed_instruction: i32,
    executed_instruction_address: usize,
}

impl<'a, I, O> Executor<'a, I, O> where I: Input, O: Output {
    pub fn new(memory: Vec<i32>, input: I, output: &'a mut O) -> Executor<'a, I, O> {
        Executor { input,
                   output,
                   memory,
                   current_address: 0,
                   debug: DebugInfo { raw_executed_instruction: 0, executed_instruction_address: 0},
                   instruction: Instruction { opcode: OpCode::Halt, parameter_modes: Vec::new() },
                 }
    }

    pub fn execute(&mut self) -> Result<(), ExecutionError> {
        while self.current_address < self.memory.len() {
            self.update_debug_state();

            self.instruction = match Instruction::new(self.memory[self.current_address]) {
                Some(instruction) => instruction,
                None => return Err(self.generate_error("invalid instruction")),
            };

            self.current_address += 1;
            match self.instruction.opcode {
                OpCode::Add => self.add_op()?,
                OpCode::Mul => self.mul_op()?,
                OpCode::Input => self.input_op()?,
                OpCode::Output => self.output_op()?,
                OpCode::JumpIfTrue => self.jump_if_true_op()?,
                OpCode::JumpIfFalse => self.jump_if_false_op()?,
                OpCode::Less => self.less_op()?,
                OpCode::Equal => self.equal_op()?,
                OpCode::Halt => break,
            };
        }

        Ok(())
    }

    fn update_debug_state(&mut self) {
        self.debug.raw_executed_instruction = self.memory[self.current_address];
        self.debug.executed_instruction_address = self.current_address;
    }

    fn generate_error(&self, msg: &str) -> ExecutionError {
        let mut e = ExecutionError::new(msg.to_string());
        e.add_info("Instruction".to_string(), format!("{}", self.debug.raw_executed_instruction));
        e.add_info("Instruction Address".to_string(), format!("{}", self.debug.executed_instruction_address));
        e
    }

    fn add_op(&mut self) -> Result<(), ExecutionError> {
        let add = |x: i32, y: i32| { x + y };
        self.basic_operation(add)
    }

    fn mul_op(&mut self) -> Result<(), ExecutionError> {
        let add = |x: i32, y: i32| { x * y };
        self.basic_operation(add)
    }

    fn input_op(&mut self) -> Result<(), ExecutionError> {
        let input = match self.input.get_input() {
            Err(e) => return Err(self.generate_error(&e.to_string())),
            Ok(input) => input,
        };

        let output_address = self.get_target_address()?;
        self.put(input, output_address)?;

        Ok(())
    }

    fn output_op(&mut self) -> Result<(), ExecutionError> {
        let param = self.get_parameters(1)?[0];
        match self.output.send_output(param) {
            Ok(()) => Ok(()),
            Err(e) => Err(self.generate_error(&e.to_string())),
        }
    }

    fn less_op(&mut self) -> Result<(), ExecutionError> {
        let func = |x: i32, y: i32| { x < y };
        self.condition_op(func)
    }

    fn equal_op(&mut self) -> Result<(), ExecutionError> {
        let func = |x: i32, y: i32| { x == y };
        self.condition_op(func)
    }

    fn condition_op<T>(&mut self, func: T) -> Result<(), ExecutionError>
        where T: Fn(i32, i32) -> bool {

        let params = self.get_parameters(2)?;
        let output_address = self.get_target_address()?;

        match func(params[0], params[1]) {
            true => self.put(1, output_address),
            false => self.put(0, output_address),
        }
    }

    fn jump_if_true_op(&mut self) -> Result<(), ExecutionError> {
        let func = |x: i32| { x != 0};
        self.jump_if(func)
    }

    fn jump_if_false_op(&mut self) -> Result<(), ExecutionError> {
        let func = |x: i32| { x == 0};
        self.jump_if(func)
    }

    fn jump_if<T>(&mut self, func: T) -> Result<(), ExecutionError>
        where T: Fn(i32) -> bool {

        let params = self.get_parameters(2)?;

        if func(params[0]) {
            let jump_address = params[1];
            self.validate_positive_address(jump_address)?;
            self.jump(jump_address as usize)?;
        }
        Ok(())
    }

    fn jump(&mut self, address: usize) -> Result<(), ExecutionError> {
        self.validate_address(address)?;
        self.current_address = address;
        Ok(())
    }

    fn basic_operation<F>(&mut self, func: F) -> Result<(), ExecutionError>
        where F: Fn(i32, i32) -> i32 {

        let params = self.get_parameters(2)?;

        let output = func(params[0],params[1]);
        let output_address = self.get_target_address()?;
        self.put(output, output_address)?;

        Ok(())
    }

    fn get_parameters(&mut self, count: usize) -> Result<Vec<i32>, ExecutionError> {
        let mut params: Vec<i32> = Vec::new();
        let mut offset = 0;
        while offset != count {
            let mode = self.get_mode(offset);
            params.push(self.get_parameter(offset, mode)?);
            offset += 1;
        }
        self.current_address += count;
        Ok(params)
    }

    fn get_target_address(&mut self) -> Result<usize, ExecutionError> {
        let output_address = self.get(self.current_address)?;
        self.validate_positive_address(output_address)?;
        self.current_address += 1;
        Ok(output_address as usize)
    }

    fn get_mode(&self, offset: usize) -> ParameterMode {
        match offset >= self.instruction.parameter_modes.len() {
            true => ParameterMode::Position,
            false => self.instruction.parameter_modes[offset].clone()
        }
    }

    fn get_parameter(&self, offset: usize, mode: ParameterMode) -> Result<i32, ExecutionError> {
        let value = self.get(self.current_address + offset)?;
        let value = match mode {
            ParameterMode::Position => {
                self.validate_positive_address(value)?;
                self.get(value as usize)?
            },
            ParameterMode::Immediate => value,
        };
        Ok(value)
    }

    fn get(&self, address: usize) -> Result<i32, ExecutionError> {
        match self.validate_address(address) {
            Ok(()) => Ok(self.memory[address]),
            Err(e) => Err(e),
        }
    }

    fn put(&mut self, val: i32, address: usize) -> Result<(), ExecutionError> {
        match self.validate_address(address) {
            Ok(()) => {
                self.memory[address] = val;
                Ok(())
            },
            Err(e) => Err(e),
        }
    }

    fn validate_address(&self, address: usize) -> Result<(), ExecutionError> {
        match address >= self.memory.len() {
            true => {
                let mut e = self.generate_error("tried accessing address out of memory range");
                e.add_info("attempted address".to_string(), format!("{}", address));
                Err(e)
            },
            false => Ok(()),
        }
    }

    fn validate_positive_address(&self, address: i32) -> Result<(), ExecutionError> {
        match address < 0 {
            true => {
                let mut e = self.generate_error("tried accessing negative address");
                e.add_info("attempted address".to_string(), format!("{}", address));
                Err(e)
            },
            false => Ok(())
        }
    }
}