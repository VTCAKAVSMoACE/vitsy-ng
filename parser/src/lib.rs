pub use crate::enrich::Operation;
pub use crate::enrich::Program;

use std::error::Error;
use std::fs::File;

mod enrich;
mod parse;

pub fn parse(name: String, code: String) -> Result<Program, Box<dyn Error>> {
    enrich::enrich(name, parse::parse(code)?)
}
