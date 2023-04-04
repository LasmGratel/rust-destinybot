use std::any::Any;
use std::marker::PhantomData;
use std::str::FromStr;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("Unknown error")]
    Unknown,

    #[error("Argument parse")]
    Argument(#[from] Box<dyn std::error::Error>)
}

pub trait Command {
    fn on_call(&self);
}

pub struct CommandManager {
    parser: Vec<Box<dyn Fn(&str) -> Result<Box<dyn Command>, ParseError>>>
}

impl CommandManager {
    pub fn parse(&mut self, s: &str) {
        self.parser.push(Box::new(PingCommand::from_str));
    }
}

pub struct PingCommand;

impl Command for PingCommand {
    fn on_call(&self) {
        todo!()
    }
}

impl PingCommand {

    fn from_str(s: &str) -> Result<Box<dyn Command>, ParseError> {
        Ok(Box::new(PingCommand))
    }
}