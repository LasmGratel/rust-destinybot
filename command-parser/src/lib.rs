use std::any::Any;
use std::collections::HashMap;
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
    fn invoke(&self);
}

pub struct CommandManager {
    parser: HashMap<String, Box<dyn Fn(&str) -> Result<Box<dyn Command>, ParseError>>>
}

impl CommandManager {

    pub fn register(&mut self, name: &str) {

    }
}
