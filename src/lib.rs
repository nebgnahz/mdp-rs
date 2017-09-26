extern crate termion;
extern crate termios;
extern crate pulldown_cmark;
extern crate textwrap;

mod deck;
mod term;
pub use deck::Deck;
pub use viewer::display;
mod viewer;
mod input;

use std::borrow::Cow;
use std::io::{self, Stdout, Write};
use termion::cursor;

// mod style;
// pub mod markdown;

trait Present {
    fn present(&self, view: &mut ViewConfig) -> io::Result<()>;
}

#[derive(Debug)]
struct ViewConfig {
    term_width: u16,
    term_height: u16,
    stdout: Stdout,

    width: u16,
    left_margin: u16,
    right_margin: u16,
    top_margin: u16,
    bottom_margin: u16,
}

impl ViewConfig {
    #[allow(dead_code)]
    pub fn new() -> io::Result<Self> {
        let (term_width, term_height) = termion::terminal_size()?;
        let width = std::cmp::min(80, term_width - 4);

        let view = ViewConfig {
            term_width: term_width,
            term_height: term_height,
            stdout: io::stdout(),

            width: width,

            left_margin: (term_width - width) / 2 - 1,
            right_margin: (term_width - width) / 2,
            top_margin: 2,
            bottom_margin: 10,
        };
        println!("{:?}", view);
        Ok(view)
    }

    pub fn clear(&mut self) -> io::Result<()> {
        write!(
            self.stdout,
            "{}{}",
            termion::clear::All,
            cursor::Goto(self.left_margin, self.top_margin)
        )
    }

    pub fn reset(&mut self) -> io::Result<()> {
        write!(self.stdout, "{}{}", termion::clear::All, cursor::Goto(1, 1))
    }

    pub fn info(&mut self) -> io::Result<()> {
        write!(self.stdout, "Your terminal is ")?;
        write!(self.stdout, "{}x{}", self.term_width, self.term_height)
    }

    pub fn newline(&mut self) -> io::Result<()> {
        write!(self.stdout, "\n{}", cursor::Right(self.left_margin - 1))
    }

    pub fn right_bottom(&self) -> (u16, u16) {
        let bottom = self.term_height - self.bottom_margin;
        let right = self.term_width - self.right_margin;
        (right, bottom)
    }

    pub fn width(&self) -> u16 {
        self.width
    }

    pub fn present<P: Present>(&mut self, p: &P) -> io::Result<()> {
        p.present(self)
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

impl Present for String {
    fn present(&self, view: &mut ViewConfig) -> io::Result<()> {
        write!(view, "{}", self)
    }
}

impl<'a> Present for Cow<'a, str> {
    fn present(&self, view: &mut ViewConfig) -> io::Result<()> {
        write!(view, "{}", self)
    }
}
