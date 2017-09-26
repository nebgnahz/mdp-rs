use std::io::{Read, Result};
use std::fs::File;
use std::path::Path;
use pulldown_cmark::Parser;
use term;

#[derive(Default)]
pub struct Deck {
    slides: Vec<Slide>,
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

        let deck = term::terminalize(&mut buf, parser);

        Ok(deck)
    }

    pub fn add(&mut self, slide: Slide) {
        self.slides.push(slide);
    }
}

#[derive(PartialEq, Eq, Clone)]
pub enum Element {
    Title(String),
    Paragraph(String),
    Quote,
    Code,
}

#[derive(Default, Clone)]
pub struct Slide {
    /// a list of all elements
    elems: Vec<Element>,
}

impl Slide {
    pub fn add(&mut self, elem: Element) {
        self.elems.push(elem);
    }

    pub fn clear(&mut self) {
        self.elems.clear();
    }
}
