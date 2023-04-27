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

impl Default for CommandManager {
    fn default() -> Self {
        CommandManager {
            parser: HashMap::new()
        }
    }
}

impl CommandManager {
    pub fn register<T>(&mut self, names: &[&str])
        where T: FromStr<Err = ParseError> + Command + 'static {
        for name in names {
            self.parser.insert(name.to_string(), Box::new(|s| {
                Ok(Box::new(T::from_str(s)?))
            }));
        }
    }
}
