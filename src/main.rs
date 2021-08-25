mod parse_args;
mod timer;

use std::num::ParseIntError;

fn main() -> Result<(), ParseIntError> {
    let args = parse_args::get_args()?;
    timer::begin(&args.seconds);
    Ok(())
}
