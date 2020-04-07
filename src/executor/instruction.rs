extern crate num;
extern crate num_derive;

use num::FromPrimitive;


#[derive(num_derive::FromPrimitive)]
pub enum OpCode {
    Add = 1,
    Mul = 2,
    Input = 3,
    Output = 4,
    JumpIfTrue = 5,
    JumpIfFalse = 6,
    Less = 7,
    Equal = 8,
    Halt = 99,
}


#[derive(num_derive::FromPrimitive, Copy, Clone)]
pub enum ParameterMode {
    Position = 0,
    Immediate = 1,
}



pub struct Instruction {
    pub opcode: OpCode,
    pub parameter_modes: Vec<ParameterMode>,
}

impl Instruction {
    pub fn new(instruction: i32) -> Option<Instruction> {
        if instruction < 0 { return None };

        let opcode: Option<OpCode> = FromPrimitive::from_i32(instruction % 100);
        let opcode = opcode?;

        let mut modes_data = (instruction / 100) as u32;
        let mut parameter_modes:Vec<ParameterMode> = Vec::new();
        while modes_data != 0 {
            let mode: Option<ParameterMode> = FromPrimitive::from_u32(modes_data % 10);

            parameter_modes.push(mode?);
            modes_data = modes_data / 10;
        }

        Some(Instruction {opcode, parameter_modes})
    }
}
