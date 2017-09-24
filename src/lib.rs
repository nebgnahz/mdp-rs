extern crate termion;
// extern crate termios;
// extern crate pulldown_cmark;
extern crate textwrap;

use std::io::{self, Stdout};
use std::io::Write;
use termion::color;
use termion::style;

// use deck::{Deck, Element, Line, Slide};
// use pulldown_cmark::{Event, Parser, Tag};

// mod input;
// mod style;

// pub mod markdown;
// pub mod deck;
// pub mod viewer;

trait Present {
    fn present(&self, view: &mut ViewConfig);
}

struct ViewConfig {
    term_width: u16,
    _term_height: u16,
    stdout: Stdout,

    width: usize,
    _height: usize,
}

impl ViewConfig {
    pub fn new() -> io::Result<Self> {
        let (width, height) = termion::terminal_size()?;
        Ok(ViewConfig {
            term_width: width,
            _term_height: height,
            stdout: io::stdout(),

            width: 80,
            _height: 60,
        })
    }
}

enum Slide {
    Title(String),
    TitleBody(String, String),
    Body(String),
}

impl Present for Slide {
    fn present(&self, view: &mut ViewConfig) {
        match self {
            &Slide::Title(ref title) => {
                // move to center
                let left = (view.term_width - title.len() as u16) / 2;
                termion::cursor::Goto(left, 0);
                write!(
                    &mut view.stdout,
                    "{}{}{}{}{}",
                    color::Fg(color::Red),
                    style::Bold,
                    title,
                    style::Reset,
                    color::Fg(color::Reset)
                );
            }
            &Slide::TitleBody(ref _title, ref body) => {
                let _lines = textwrap::fill(&body, view.width);
            }
            &Slide::Body(ref _body) => {}
        }
    }
}
