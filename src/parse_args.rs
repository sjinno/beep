use super::PathBuf;

use text_colorizer::Colorize;

use std::env;
use std::fs;

#[derive(Debug)]
pub struct Arguments {
    pub path_to_audio: PathBuf,
    pub seconds: u64,
}

fn print_usage() {
    eprintln!("Usage: buzz <path-to-audiofile> <minutes> <seconds>");
}

pub fn get_args() -> Result<Arguments, anyhow::Error> {
    let args = env::args().skip(1).collect::<Vec<_>>();

    if args.is_empty() || args.len() == 1 || args.len() > 3 {
        print_usage();
        eprintln!(
            "{} wrong number of arguments: expected 2 or 3, got {}.",
            "Error:".red().bold(),
            args.len()
        );
        std::process::exit(1);
    }

    parse_args(&args)
}

fn parse_args(args: &[String]) -> Result<Arguments, anyhow::Error> {
    let path_to_audio = fs::canonicalize(PathBuf::from(&args[0]))?;
    // println!("{:?}", path_to_audio);
    let min = args[1].parse::<u64>()?;
    let sec = if args.len() == 3 {
        Some(args[2].parse::<u64>()?)
    } else {
        None
    };
    let seconds = if let Some(s) = sec {
        min * 60 + s
    } else {
        min * 60
    };
    Ok(Arguments {
        path_to_audio,
        seconds,
    })
}
