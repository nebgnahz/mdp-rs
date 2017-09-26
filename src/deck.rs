use Present;
use ViewConfig;
use pulldown_cmark::Parser;
use std::fs::File;
use std::io::{self, Read, Write};
use std::path::Path;
use term;
use termion::{color, style};
use textwrap;

#[derive(Default, Debug)]
pub struct Deck {
    slides: Vec<Slide>,
    current: usize,
}

fn read_markdown<P: AsRef<Path>>(path: P) -> io::Result<String> {
    let mut f = File::open(path)?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

impl Deck {
    pub fn from_path<P: AsRef<Path>>(path: P) -> io::Result<Deck> {
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

    pub fn total_num(&self) -> usize {
        self.slides.len()
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
    fn present(&self, view: &mut ViewConfig) -> io::Result<()> {
        for elem in &self.elems {
            match elem {
                &Element::Title(ref _title) => {
                    // let left = view.width() / 2 - title.len() as u16 / 2;
                    // write!(view, "{}", termion::cursor::Goto(left, 2))?;
                }
                _ => {}
            }
            view.present(elem)?;
            view.newline()?;
        }
        Ok(())
    }
}

impl Present for Element {
    fn present(&self, view: &mut ViewConfig) -> io::Result<()> {
        match self {
            &Element::Title(ref title) => {
                write!(view, "{}{}", color::Fg(color::Red), style::Bold)?;
                write!(view, "{}", title)?;
                write!(view, "{}{}", style::Reset, color::Fg(color::Reset))?;
            }
            &Element::Paragraph(ref content) => {
                let cols = view.width() as usize;
                let lines = textwrap::Wrapper::new(cols).wrap(content);
                for ref l in lines {
                    view.newline()?;
                    view.present(l)?;
                }
            }
            &Element::Code(ref content) => {
                let cols = view.width() as usize;
                write!(view, "{}", color::Bg(color::White))?;
                write!(view, "{}", color::Fg(color::Black))?;
                for ref line in content.split('\n') {
                    view.newline()?;
                    view.present(line)?;

                    let to_fill = cols - line.len();
                    let fill = (0..to_fill).map(|_| ' ').collect::<String>();
                    view.present(&fill)?;
                    view.present(&"another line")?;
                }
                write!(view, "{}", color::Bg(color::Reset))?;
                write!(view, "{}", color::Fg(color::Reset))?;
            }
            _ => {}
        }
        Ok(())
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
