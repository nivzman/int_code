
use int_code::{self, parsing};
use int_code::input::FixedInput;
use int_code::output::QueueOutput;


pub struct Combinations<T> where T: Clone {
    items: Vec<T>,
    current: usize,
    base_gen: Option<Box<Combinations<T>>>,
}

impl<T> Combinations<T> where T: Clone {
    pub fn new(items: Vec<T>) -> Combinations<T> {
        Combinations {items, current: 0, base_gen: None}
    }
}

impl<T> Iterator for Combinations<T> where T: Clone {
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let None = self.base_gen {
                if self.current >= self.items.len() {
                    return None;
                }

                if self.items.len() == 1 {
                    self.current += 1;
                    return Some(self.items.clone());
                }

                let mut items = self.items.clone();
                items.remove(self.current);
                self.base_gen = Some(Box::new(Combinations::new(items)));
            }

            match self.base_gen.as_mut().unwrap().next() {
                None => {
                    self.current += 1;
                    self.base_gen = None;
                },
                Some(mut combination) => {
                    combination.insert(0, self.items[self.current].clone());
                    return Some(combination);
                },
            }
        }
    }
}



fn run_with_phases(memory: Vec<i32>, phases: Vec<i32>) -> Result<i32, Box<dyn std::error::Error>> {
    let mut input = 0;
    for phase in phases.into_iter() {
        let mut output = QueueOutput::new();
        int_code::execute(memory.clone(), FixedInput::new(vec![phase, input]), &mut output)?;
        input = output.get().expect("no output");
    }
    Ok(input)
}


#[test]
fn solve1() -> Result<(), Box<dyn std::error::Error>> {
    let memory = parsing::load_file("tests/programs/stage7.txt")?;

    let mut max_output = i32::min_value();
    let combinations = Combinations::new(vec![0, 1, 2, 3, 4]);
    for combination in combinations {
        let output = run_with_phases(memory.clone(), combination)?;
        if output > max_output {
            max_output = output;
        }
    }

    assert_eq!(max_output, 43812);

    Ok(())
}