extern crate termion;
// extern crate termios;
extern crate pulldown_cmark;
extern crate textwrap;

use std::io::{self, Stdout};
use std::io::{Read, Result};
// use termion::color;
// use termion::style;
use std::fs::File;
use std::path::Path;

use pulldown_cmark::{Parser, Event, Tag};

fn read_markdown<P: AsRef<Path>>(path: P) -> Result<String> {
    let mut f = File::open(path)?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

#[allow(dead_code)]
enum Element {
    Title(String),
    Body(String),
    Bullets(String),
    Code(String),
    Image(String),
}

fn parse_to_deck(text: &str, deck: &mut Deck) -> Result<()> {
    // let mut current_tag: Option<Element> = None;
    let mut content: String = String::new();

    let mut elements: Vec<Tag> = Vec::new();

    let mut parser = Parser::new(text);
    loop {
        let event = match parser.next() {
            None => break,
            Some(e) => e,
        };

        println!("{:?}", event);

        match event {
            Event::Start(tag) => {
                match tag {
                    Tag::Rule => {
                        let offset = parser.get_offset();
                        let slide = Slide::new(elements.clone(), offset);
                        elements.clear();
                        deck.add(slide);
                    }
                    _ => {}
                }
            }
            Event::Text(text) => content.push_str(&text),
            Event::End(_tag) => {}
            _ => {}
        }
    }
    Ok(())
}

#[derive(Default)]
pub struct Deck {
    slides: Vec<Slide>,
}

impl Deck {
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Deck> {
        let content = read_markdown(path)?;
        let mut deck = Deck::default();
        parse_to_deck(&content, &mut deck)?;
        Ok(deck)
    }

    fn add(&mut self, slide: Slide) {
        self.slides.push(slide);
    }
}

#[allow(dead_code)]
struct Slide {
    /// a list of all elements
    elems: Vec<Element>,

    /// The slide style
    style: Style,

    /// The offset with respect in the buffer
    offset: usize,
}

impl Slide {
    fn new(_elems: Vec<Tag>, offset: usize) -> Slide {
        Slide {
            elems: Vec::new(),
            style: Style::Title,
            offset: offset,
        }
    }
}

#[allow(dead_code)]
enum Style {
    Title,
    TitleBody,
    Body,
}

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

#[allow(dead_code)]
struct ViewConfig {
    term_width: u16,
    _term_height: u16,
    stdout: Stdout,

    width: usize,
    _height: usize,
}

impl ViewConfig {
    #[allow(dead_code)]
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

// impl Present for Slide {
//     fn present(&self, view: &mut ViewConfig) {
//         match self {
//             &Slide::Title(ref title) => {
//                 // move to center
//                 let left = (view.term_width - title.len() as u16) / 2;
//                 termion::cursor::Goto(left, 0);
//                 write!(
//                     &mut view.stdout,
//                     "{}{}{}{}{}",
//                     color::Fg(color::Red),
//                     style::Bold,
//                     title,
//                     style::Reset,
//                     color::Fg(color::Reset)
//                 );
//             }
//             &Slide::TitleBody(ref _title, ref body) => {
//                 let _lines = textwrap::fill(&body, view.width);
//             }
//             &Slide::Body(ref _body) => {}
//         }
//     }
// }
