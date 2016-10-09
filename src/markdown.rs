use std::string::String;
use std::fmt::{Formatter, Display, Result};
use termion::color;
use termion::style;

pub enum Element {
    H1,
    Paragraph,
    Quote,
    Code,
}

pub struct Line {
    pub text: String,
    pub elem: Element,
}

impl Display for Line {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self.elem {
            Element::H1 => {
                try!(write!(f,
                            "{}{}{}{}{}",
                            color::Fg(color::Red),
                            style::Bold,
                            self.text,
                            style::Reset,
                            color::Fg(color::Reset)));

            }
            Element::Paragraph => {
                try!(write!(f, "{}", self.text));
            }
            Element::Quote => {
                try!(write!(f, "{}{}{}", style::Italic, self.text, style::Reset));
            }
            _ => {}
        }
        Ok(())
    }
}

pub struct Slide {
    pub lines: Vec<Line>,
}

pub struct Deck {
    pub slides: Vec<Slide>,
    pub current: usize,
}

impl Deck {
    pub fn slide(&self) -> &Slide {
        &self.slides[self.current]
    }

    pub fn previous(&mut self) {
        if self.current > 0 {
            self.current -= 1;
        }
    }

    pub fn next(&mut self) {
        if self.current < (self.slides.len() - 1) {
            self.current += 1;
        }
    }
}

/// Create a demo deck
pub fn demo() -> Deck {
    let slide1 = Slide {
        lines: vec![Line {
                        text: "Slide 1".to_string(),
                        elem: Element::H1,
                    },
                    Line {
                        text: "Paragraph on Page 1".to_string(),
                        elem: Element::Paragraph,
                    }],
    };
    let slide2 = Slide {
        lines: vec![Line {
                        text: "Hello Slide 2".to_string(),
                        elem: Element::H1,
                    },
                    Line {
                        text: "A great quote from someone".to_string(),
                        elem: Element::Quote,
                    }],
    };
    Deck {
        slides: vec![slide1, slide2],
        current: 0,
    }
}
