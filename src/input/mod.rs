use std::io;
use std::sync::mpsc::Receiver;
use std::time::Duration;

use crate::errors::InputError;

pub trait Input {
    fn get_input(&mut self) -> Result<i32, Box<dyn std::error::Error>>;
}



pub struct UserInput {}

impl Input for UserInput {
    fn get_input(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let mut input = String::new();
        println!("input : ");
        io::stdin().read_line(&mut input)?;
        let input:i32 = input.trim().parse()?;
        Ok(input)
    }
}



pub struct PipeInput {
    receiving_end: Receiver<i32>,
    max_wait_time: Duration,
}

impl Input for PipeInput {
    fn get_input(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        let input = self.receiving_end.recv_timeout(self.max_wait_time)?;
        Ok(input)
    }
}



pub struct FixedInput {
    inputs: Vec<i32>,
    current: usize,
}

impl FixedInput {
    pub fn new(inputs: Vec<i32>) -> FixedInput {
        FixedInput {inputs, current: 0}
    }
}

impl Input for FixedInput {
    fn get_input(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        match self.current < self.inputs.len() {
            true => {
                let ret = self.inputs[self.current];
                self.current += 1;
                Ok(ret)
            },
            false => Err(Box::new(InputError::new("no more inputs")))
        }
    }
}



pub struct ExtendedFixedInput<I> where I: Input {
    extending_input: I,
    pre_inputs: Vec<i32>,
    current: usize,
}

impl<I> ExtendedFixedInput<I> where I: Input {
    pub fn new(base_input: I, pre_inputs: Vec<i32>) -> ExtendedFixedInput<I> {
        ExtendedFixedInput { extending_input: base_input, pre_inputs, current: 0}
    }
}

impl<I> Input for ExtendedFixedInput<I> where I: Input {
    fn get_input(&mut self) -> Result<i32, Box<dyn std::error::Error>> {
        match self.current < self.pre_inputs.len() {
            true => {
                let input = self.pre_inputs[self.current];
                self.current += 1;
                Ok(input)
            }
            false => self.extending_input.get_input()
        }
    }
}