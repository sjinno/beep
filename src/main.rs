mod parse_args;
mod timer;

use parse_args::Arguments;

use std::path::PathBuf;

fn main() -> Result<(), anyhow::Error> {
    let args = parse_args::get_args()?;
    timer::begin(&args);
    Ok(())
}
