use std::error;
use std::fmt;
use std::collections::HashMap;


#[derive(Debug, PartialEq)]
pub struct ParseError {
    msg: String,
    unparseable: String,
}

impl ParseError {
    pub fn new(msg: String, unparseable: String) -> ParseError {
        ParseError {msg, unparseable}
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} : {}", self.msg, self.unparseable)
    }
}

impl error::Error for ParseError {}


#[derive(Debug, PartialEq)]
pub struct ExecutionError {
    msg: String,
    additional_info: HashMap<String, String>,
}

impl ExecutionError {
    pub fn new(msg: String) -> ExecutionError {
        ExecutionError { msg, additional_info: HashMap::new() }
    }

    pub fn add_info(&mut self, key: String, val: String) {
        self.additional_info.insert(key, val);
    }
}

impl error::Error for ExecutionError {}

impl fmt::Display for ExecutionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}\n    {:?}", self.msg, self.additional_info)
    }
}


#[derive(Debug, PartialEq)]
pub struct InputError {
    msg: String,
}

impl InputError {
    pub fn new(msg: &str) -> InputError {
        InputError {msg: msg.to_string()}
    }
}

impl fmt::Display for InputError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl error::Error for InputError {}