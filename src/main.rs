mod one_1;
mod one_2;

use std::io::Result;
use one_2 as on;

fn main() -> Result<()> {
    on::main()?;

    Ok(())
}
