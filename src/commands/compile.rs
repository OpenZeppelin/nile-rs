pub mod cairo_to_sierra;
pub mod sierra_to_casm;

use anyhow::{Ok, Result};

/// Common trait for all compilers
pub trait Compiler {
    type Output;
    fn run(self) -> Result<Self::Output>;
}

/// Nile compile command
pub fn compile() -> Result<()> {
    println!("Compiling!");
    Ok(())
}
