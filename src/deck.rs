use std::fmt::{Display, Formatter, Result};
use std::string::String;
use style::{code_fill_column, text_fill_column};
use termion::color;
use termion::style;

/// Deck holds a vector of slides
#[derive(Default)]
pub struct Deck {
    pub slides: Vec<Slide>,
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Element {
    H1,
    Paragraph,
    Quote,
    Code,

    Rule,
}

#[derive(Clone)]
pub struct Line {
    pub text: String,
    pub elem: Element,
}

impl Display for Line {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self.elem {
            Element::H1 => {
                write!(
                    f,
                    "{}{}{}{}{}",
                    color::Fg(color::Red),
                    style::Bold,
                    self.text,
                    style::Reset,
                    color::Fg(color::Reset)
                )?
            }
            Element::Paragraph => {
                let lines = text_fill_column(&self.text, 80);
                for line in lines {
                    try!(writeln!(f, "{}", line));
                }
            }
            Element::Quote => {
                // Italic
                try!(write!(f, "{}", style::Italic));
                for line in text_fill_column(&self.text, 80) {
                    try!(writeln!(f, "{}", line));
                }
                try!(write!(f, "{}", style::Reset));
            }
            Element::Code => {
                try!(write!(f, "{}", color::Bg(color::White)));
                try!(write!(f, "{}", color::Fg(color::Black)));
                for line in code_fill_column(&self.text, 80) {
                    try!(writeln!(f, "{}", line));
                }
                try!(write!(f, "{}", color::Fg(color::Reset)));
                try!(write!(f, "{}", color::Bg(color::Reset)));
            }
            _ => unreachable!{},
        }
        Ok(())
    }
}

#[derive(Default, Clone)]
pub struct Slide {
    pub lines: Vec<Line>,
}

impl Slide {
    pub fn add_line(&mut self, line: Line) {
        self.lines.push(line);
    }

    pub fn clear(&mut self) {
        self.lines.clear();
    }
}


impl Deck {
    pub fn add_slide(&mut self, slide: Slide) {
        self.slides.push(slide);
    }

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

const DEMO_CODE: &'static str = r#"#include <stdio.h>

int main() {
    printf(\"Hello World\");
}"#;

/// Create a demo deck
pub fn demo() -> Deck {
    let slide1 = Slide {
        lines: vec![
            Line {
                text: "Slide 1".to_string(),
                elem: Element::H1,
            },
            Line {
                text: "Lorem Ipsum is simply dummy text of the printing and typesetting \
                               industry. Lorem Ipsum has been the industry's standard dummy text \
                               ever since the 1500s, when an unknown printer took a galley of \
                               type and scrambled it to make a type specimen book. It has \
                               survived not only five centuries, but also the leap into \
                               electronic typesetting, remaining essentially unchanged. It was \
                               popularised in the 1960s with the release of Letraset sheets \
                               containing Lorem Ipsum passages, and more recently with desktop \
                               publishing software like Aldus PageMaker including versions of \
                               Lorem Ipsum."
                    .to_string(),
                elem: Element::Paragraph,
            },
        ],
    };

    let slide2 = Slide {
        lines: vec![
            Line {
                text: "Hello Slide 2".to_string(),
                elem: Element::H1,
            },
            Line {
                text: "Start by doing what's necessary; then do what's possible; and \
                               suddenly you are doing the impossible."
                    .to_string(),
                elem: Element::Quote,
            },
        ],
    };

    let slide3 = Slide {
        lines: vec![
            Line {
                text: "Hello Slide 3".to_string(),
                elem: Element::H1,
            },
            Line {
                text: DEMO_CODE.to_string(),
                elem: Element::Code,
            },
        ],
    };

    Deck {
        slides: vec![slide1, slide2, slide3],
        current: 0,
    }
}
