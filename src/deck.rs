use Present;
use ViewConfig;
use pulldown_cmark::Parser;
use std::fs::File;
use std::io::{Read, Result, Write};
use std::path::Path;
use term;
use termion;

#[derive(Default, Debug)]
pub struct Deck {
    slides: Vec<Slide>,
    current: usize,
}

fn read_markdown<P: AsRef<Path>>(path: P) -> Result<String> {
    let mut f = File::open(path)?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

impl Deck {
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Deck> {
        let content = read_markdown(path)?;
        let parser = Parser::new(&content);
        let mut buf = String::new();

        Ok(term::terminalize(&mut buf, parser))
    }

    pub fn add(&mut self, slide: Slide) {
        self.slides.push(slide);
    }

    pub fn next(&mut self) {
        if self.current < self.slides.len() - 1 {
            self.current += 1;
        }
    }

    pub fn previous(&mut self) {
        if self.current > 0 {
            self.current -= 1;
        }
    }

    pub fn slide(&self) -> &Slide {
        &self.slides[self.current]
    }

    pub fn current_num(&self) -> usize {
        self.current
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Element {
    Title(String),
    Paragraph(String),
    Quote(String),
    Code(String),
}

#[derive(Default, Clone, Debug)]
pub struct Slide {
    /// a list of all elements
    elems: Vec<Element>,
}

impl Present for Slide {
    fn present(&self, view: &mut ViewConfig) {
        for elem in &self.elems {
            match elem {
                &Element::Title(ref title) => {
                    // calculate the center
                    let left = view.width() / 2 - title.len() as u16 / 2;
                    write!(view, "{}", termion::cursor::Goto(left, 1)).unwrap();
                    write!(view, "{}", title).unwrap();
                }
                _ => {}
            }
        }
    }
}

impl Slide {
    pub fn add(&mut self, elem: Element) {
        self.elems.push(elem);
    }

    pub fn clear(&mut self) {
        self.elems.clear();
    }
}
