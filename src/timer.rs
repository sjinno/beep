use rodio::{self, OutputStreamHandle};
use std::fmt::Write;

type AudioFileResult = std::io::Result<()>;

pub fn begin(dur: &std::time::Duration) {
    std::thread::sleep(*dur);
    let (_stream, stream_handle) = rodio::OutputStream::try_default().unwrap();
    match buzz(&stream_handle) {
        Ok(buzz) => buzz,
        Err(_) => println!("No such audio file found."),
    }
}

fn buzz(stream_handle: &OutputStreamHandle) -> AudioFileResult {
    let file = std::fs::File::open("audio/hahn_kikeriki.mp3")?;
    let sound = stream_handle
        .play_once(std::io::BufReader::new(file))
        .unwrap();
    sound.set_volume(0.5);
    sound.sleep_until_end();
    Ok(())
}

#[allow(dead_code)]
fn print_progress() {
    unimplemented!();
    let mut timer = String::new();
    let world = "world";
    write!(&mut timer, "Hello, {}!\r", world)
        .expect("Error ocuured while trying to write in String.");
    print!("{}", timer);
}
