
mod executor;
mod errors;
pub mod parsing;
pub mod input;
pub mod output;

use input::Input;
use output::Output;
use errors::ExecutionError;
use executor::Executor;

pub fn execute_file<I, O>(file: &str, input: I, output: &mut O) -> Result<(), Box<dyn std::error::Error>>
    where I: Input, O: Output {

    let memory = parsing::load_file(file)?;
    execute(memory, input, output)?;
    Ok(())
}

pub fn execute<I, O>(memory: Vec<i32>, input: I, output: &mut O) -> Result<(), ExecutionError>
    where I: Input, O: Output {

    let mut executor = Executor::new(memory, input, output);
    executor.execute()
}