extern crate termion;
extern crate termios;
extern crate pulldown_cmark;
// extern crate textwrap;

mod deck;
mod term;
pub use deck::Deck;
pub use viewer::display;
mod viewer;
mod input;

use std::io::{self, Stdout, Write};

// mod style;

// pub mod markdown;
// pub mod deck;


trait Present {
    fn present(&self, view: &mut ViewConfig);
}

#[allow(dead_code)]
struct ViewConfig {
    term_width: u16,
    pub term_height: u16,
    stdout: Stdout,

    width: u16,
}

impl ViewConfig {
    #[allow(dead_code)]
    pub fn new() -> io::Result<Self> {
        let (width, height) = termion::terminal_size()?;
        Ok(ViewConfig {
            term_width: width,
            term_height: height,
            stdout: io::stdout(),

            width: 80,
        })
    }

    pub fn _height(&self) -> u16 {
        self.term_height
    }

    pub fn width(&self) -> u16 {
        self.width
    }
}

impl Write for ViewConfig {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.stdout.write(buf)
    }
    fn flush(&mut self) -> io::Result<()> {
        self.stdout.flush()
    }
}
