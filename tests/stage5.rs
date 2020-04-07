use std::error;

use int_code::{self, parsing};
use int_code::input::FixedInput;
use int_code::output::QueueOutput;


#[test]
fn solve1() -> Result<(), Box<dyn error::Error>> {
    let mem = parsing::load_file("tests/programs/stage5.txt")?;
    let input = FixedInput::new(vec![1]);
    let mut output = QueueOutput::new();
    int_code::execute(mem, input, &mut output)?;

    for _ in 0..9 {
        assert_eq!(output.get().unwrap(), 0);
    }
    assert_eq!(output.get().unwrap(), 5346030);

    Ok(())
}

#[test]
fn solve2() -> Result<(), Box<dyn error::Error>>{
    let mem = parsing::load_file("tests/programs/stage5.txt")?;
    let input = FixedInput::new(vec![5]);
    let mut output = QueueOutput::new();
    int_code::execute(mem, input, &mut output)?;

    assert_eq!(output.get().unwrap(), 513116);

    Ok(())
}