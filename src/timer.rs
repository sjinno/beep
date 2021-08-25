use super::{Arguments, PathBuf};

use chrono::{Duration as ChronoDuration, NaiveTime};
use rodio::{self, OutputStreamHandle};

use std::convert::TryInto;
use std::fs;
use std::io::{self, Write as IoWrite};
use std::thread;
use std::time::Duration;

type AudioFileResult = io::Result<()>;

pub fn begin(args: &Arguments) {
    countdown(args.seconds);

    let (_stream, stream_handle) = rodio::OutputStream::try_default().unwrap();
    match buzz(&args.path_to_audio, &stream_handle) {
        Ok(buzz) => buzz,
        Err(_) => {}
    }
}

fn buzz(path_to_audio: &PathBuf, stream_handle: &OutputStreamHandle) -> AudioFileResult {
    let file = fs::File::open(path_to_audio)?;
    let sound = stream_handle.play_once(io::BufReader::new(file)).unwrap();
    sound.set_volume(0.5);
    sound.sleep_until_end();
    println!();
    Ok(())
}

fn get_initial_state(
    sec: u64,
) -> Result<NaiveTime, Box<dyn std::error::Error + Send + Sync + 'static>> {
    let mut sec: u32 = sec.try_into()?;
    let h = sec.div_euclid(3600);
    sec -= h * 3600;
    let m = sec.div_euclid(60);
    let s = sec - m * 60;

    Ok(NaiveTime::from_hms(h, m, s))
}

fn countdown(seconds: u64) {
    let mut state = get_initial_state(seconds).unwrap_or_else(|_| NaiveTime::from_hms(0, 0, 0));
    print!("{}", state);
    io::stdout().flush().unwrap();

    let done = NaiveTime::from_hms(0, 0, 0);
    let dur = Duration::from_secs(1);
    let chrono_dur = ChronoDuration::seconds(1);
    loop {
        thread::sleep(dur);
        state -= chrono_dur;
        print!("\r{}", state);
        io::stdout().flush().unwrap();

        if state == done {
            break;
        }
    }
}
