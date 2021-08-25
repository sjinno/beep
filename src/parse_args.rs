use super::ParseIntError;
use std::{env, time::Duration};
use text_colorizer::Colorize;

#[derive(Debug)]
pub struct Arguments {
    pub seconds: Duration,
}

fn print_usage() {
    eprintln!("Usage: buzz <minutes> <seconds>");
}

pub fn get_args() -> Result<Arguments, ParseIntError> {
    let args = env::args().skip(1).collect::<Vec<_>>();

    if args.is_empty() || args.len() > 2 {
        print_usage();
        eprintln!(
            "{} wrong number of arguments: expected 1 or 2, got {}.",
            "Error:".red().bold(),
            args.len()
        );
        std::process::exit(1);
    }

    parse_args(&args)
}

fn parse_args(args: &[String]) -> Result<Arguments, ParseIntError> {
    let min = args[0].parse::<u64>()?;
    let sec = if args.len() == 2 {
        Some(args[1].parse::<u64>()?)
    } else {
        None
    };
    let duration_in_secs = if let Some(s) = sec {
        min * 60 + s
    } else {
        min * 60
    };
    Ok(Arguments {
        seconds: Duration::from_secs(duration_in_secs),
    })
}
