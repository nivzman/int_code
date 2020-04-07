use std::collections::VecDeque;
use std::sync::mpsc::Sender;

pub trait Output {
    fn send_output(&mut self, output: i32) -> Result<(), Box<dyn std::error::Error>>;
}


pub struct ScreenOutput {}

impl Output for ScreenOutput {
    fn send_output(&mut self, output: i32) -> Result<(), Box<dyn std::error::Error>>{
        println!("{}", output);
        Ok(())
    }
}



pub struct QueueOutput {
    outputs: VecDeque<i32>,
}

impl QueueOutput {
    pub fn new() -> QueueOutput {
        QueueOutput {outputs: VecDeque::new()}
    }

    pub fn get(&mut self) -> Option<i32> {
        self.outputs.pop_front()
    }
}

impl Output for QueueOutput {
    fn send_output(&mut self, output: i32) -> Result<(), Box<dyn std::error::Error>>{
        self.outputs.push_back(output);
        Ok(())
    }
}



pub struct PipeOutput {
    sending_end: Sender<i32>,
}

impl Output for PipeOutput {
    fn send_output(&mut self, output: i32) -> Result<(), Box<dyn std::error::Error>> {
        Ok(self.sending_end.send(output)?)
    }
}