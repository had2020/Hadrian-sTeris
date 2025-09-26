use game::logic::End;
use tetrs::Tetrs;

mod game;
mod input;
mod tetrs;
mod ui;

use std::fs::File;
use std::io::BufReader;
use std::thread;
use std::time::Duration;

fn main() {
    thread::spawn(|| loop {
        let stream_handle =
            rodio::OutputStreamBuilder::open_default_stream().expect("open default audio stream");
        let file = BufReader::new(File::open("sounds/0x06.mp3").unwrap());
        let sink = rodio::play(&stream_handle.mixer(), file).unwrap();
        thread::sleep(Duration::from_secs(52));
    });

    let game = Tetrs::new();
    while game.run() != End::Quit {}
}
