use std::io;
use std::io::stdin;
use input::ImmediateInput;
use termion::input::TermRead;
use termion::event::Key;

#[allow(dead_code)]
struct Options {}

pub fn display() -> io::Result<()> {
    // We modify the terminal to take immediate result.
    let input = ImmediateInput::new(0);
    input.set_immediate();

    loop {
        // Read user input
        let stdin = stdin();
        // let mut buf = [0; 1];
        // println!("{:?}", buf);
        for c in stdin.keys() {
            match c.unwrap() {
                Key::Down | Key::Char('j') => {
                    println!("next slide");
                }
                Key::Up | Key::Char('k') => {
                    println!("previous slide");
                }
                _ => {
                    println!("Other key");
                }
            }
        }
    }
}
