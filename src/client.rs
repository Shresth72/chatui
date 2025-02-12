use crossterm::cursor::MoveTo;
use crossterm::event::{poll, read, Event};
use crossterm::terminal::{self, ClearType};
use crossterm::QueueableCommand;
use std::io::{stdout, Write};
use std::thread;
use std::time::Duration;

fn main() {
    let mut stdout = stdout();
    let (mut w, mut h) = terminal::size().unwrap();
    let mut bar = "-".repeat(w as usize);

    loop {
        while poll(Duration::ZERO).unwrap() {
            match read().unwrap() {
                Event::Resize(nw, nh) => {
                    w = nw;
                    h = nh;
                    bar = "-".repeat(w as usize);
                }
                Event::Key(_event) => {}
                _ => {}
            }
        }

        stdout.queue(terminal::Clear(ClearType::All)).unwrap();
        stdout.queue(MoveTo(0, h - 2)).unwrap();
        stdout.write(bar.as_bytes()).unwrap();
        stdout.queue(MoveTo(0, h - 1)).unwrap();
        stdout.flush().unwrap();

        thread::sleep(Duration::from_millis(33));
    }
}
