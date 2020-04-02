use std::env::args;
use std::error::Error;

#[cfg(feature = "emulation")]
#[inline]
fn run(code: String) -> Result<(), Box<dyn Error>> {
    emulator::run(code)
}

#[cfg(feature = "jit")]
#[inline]
fn run(code: String) -> Result<(), Box<dyn Error>> {
    compiler::run(code)
}

fn main() -> Result<(), Box<dyn Error>> {
    run(args()
        .skip(1)
        .next()
        .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::Other, "No file specified."))?)?;
    Ok(())
}
