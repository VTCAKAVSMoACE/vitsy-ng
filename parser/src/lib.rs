pub use crate::parse::{Method, Program};
use std::cell::RefCell;
use std::error::Error;
use std::rc::Rc;

mod parse;

pub fn parse(name: String) -> Result<Rc<RefCell<Program>>, Box<dyn Error>> {
    parse::parse(name)
}

#[cfg(test)]
mod test;
