use std::any::Any;
use std::marker::PhantomData;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("Unknown error")]
    Unknown
}

struct CommandSpec {

}

pub struct ArgumentSpec<T, R> {
    _phantom: PhantomData<T>,
    _remaining: PhantomData<R>,
}

impl<T, R> Default for ArgumentSpec<T, R> {
    fn default() -> Self {
        ArgumentSpec {
            _phantom: PhantomData::default(),
            _remaining: PhantomData::default()
        }
    }
}

pub struct Cons<T> {
    _phantom: PhantomData<T>,
}

impl<T> Default for Cons<T> {
    fn default() -> Self {
        Cons {
            _phantom: PhantomData::default()
        }
    }
}


pub struct Nil;

macro_rules! command {
    ($name:tt, $($arg:tt:$t:ty),+) => {
        register_command($name.to_string());
        dbg!("Command {} with args", $name, ($($t::type_id()),+));
    };
}

fn register_command(name: String) {
    let a: ArgumentSpec<Cons<String>, Nil> = ArgumentSpec::default();
}

fn on_call() {
    //command!("bla", name: String);
}

fn case<'a, 'b, F>(literal: &'a str, f: F) where F: Fn(&'b str) {

}