use parser::parse;
use std::error::Error;

pub fn run(name: String) -> Result<(), Box<dyn Error>> {
    let _program = parse(name)?;
    unimplemented!()
}
