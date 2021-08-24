// use anyhow::Result;
use text_colorizer::Colorize;

use std::{env, num::ParseIntError, time::Duration};

#[derive(Debug)]
struct Arguments {
    seconds: Duration,
}

fn print_usage() {
    eprintln!("Usage: buzz <minutes> <seconds>");
}

fn parse_args(args: &Vec<String>) -> Result<Arguments, ParseIntError> {
    let min = args[0].parse::<u64>()?;
    let sec = if args.len() == 2 {
        Some(args[1].parse::<u64>()?)
    } else {
        None
    };
    let duration_in_secs = if sec.is_some() {
        min * 60 + sec.unwrap()
    } else {
        min * 60
    };
    Ok(Arguments {
        seconds: Duration::from_secs(duration_in_secs),
    })
}

fn main() {
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

    let args = parse_args(&args);
    println!("{:#?}", args);
}
