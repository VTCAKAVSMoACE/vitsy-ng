use std::env::args;
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::path::Path;

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
        .map(|name| {
            File::open(Path::new(&name)).map(|mut f| {
                let mut contents = String::new();
                match f.read_to_string(&mut contents) {
                    Ok(_) => Ok(contents),
                    Err(e) => Err(e),
                }
            })
        })
        .ok_or_else(|| {
            std::io::Error::new(std::io::ErrorKind::Other, "No file specified.")
        })???)?;
    Ok(())
}
