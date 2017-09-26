use Present;
use ViewConfig;
use pulldown_cmark::{Parser, Tag};
use pulldown_cmark::Event::{self, End, Html, InlineHtml, Start, Text};
use pulldown_cmark::Event::{FootnoteReference, HardBreak, SoftBreak};

use split;
use std::borrow::Cow;
use std::io::{self, Write};
use termion::{color, style};
use textwrap;

#[derive(Default, Debug)]
pub struct Deck {
    slides: Vec<Slide>,
    current: usize,
}

#[derive(Default, Debug)]
pub struct Deck2<'a> {
    slides: Vec<Slide2<'a>>,
    current: usize,
}

#[derive(Default, Debug)]
pub struct Slide2<'a> {
    content: Cow<'a, str>,
    offset: usize,
}

impl<'a> Slide2<'a> {
    pub fn new(offset: usize, content: Cow<'a, str>) -> Self {
        Slide2 {
            content: content,
            offset: offset,
        }
    }
}

impl<'a> Present for Slide2<'a> {
    fn present(&self, view: &mut ViewConfig) -> io::Result<()> {
        let parser = Parser::new(&self.content);
        for element in parser {
            println!("{:?}", element);
        }

        let parser = Parser::new(&self.content);
        for element in parser {
            view.present(&element)?;
        }
        Ok(())
    }
}

impl<'a> Present for Event<'a> {
    fn present(&self, view: &mut ViewConfig) -> io::Result<()> {
        match self {
            &Start(Tag::Emphasis) => view.start_italic(),
            &End(Tag::Emphasis) => view.end_italic(),
            &Start(Tag::Strong) => view.start_bold(),
            &End(Tag::Strong) => view.end_bold(),
            &Start(Tag::Code) => view.start_code(),
            &End(Tag::Code) => view.end_code(),
            &SoftBreak => view.newline(),
            &Start(Tag::Header(level)) => view.start_header(level),
            &End(Tag::Header(level)) => view.end_header(level),
            &Start(Tag::CodeBlock(ref _lang)) => view.start_codeblock(),
            &End(Tag::CodeBlock(ref _lang)) => view.end_codeblock(),
            &Start(Tag::Paragraph) => view.start_paragraph(),
            &End(Tag::Paragraph) => view.end_paragraph(),
            &Start(_) => Ok(()),
            &End(_) => Ok(()),
            &Text(ref text) => view.show_text(text),
            &Html(ref _html) |
            &InlineHtml(ref _html) => unimplemented!{},
            &FootnoteReference(ref _ref) => unimplemented!{},
            &HardBreak => Ok(()),
        }
    }
}

impl<'a> Deck2<'a> {
    pub fn new(content: &'a str) -> io::Result<Deck2<'a>> {
        let slides = split::split(content)
            .map(|s| Slide2::new(s.0, s.1))
            .collect::<Vec<_>>();

        let deck2 = Deck2 {
            slides: slides,
            current: 0,
        };

        Ok(deck2)
    }

    pub fn add(&mut self, slide: Slide2<'a>) {
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

    pub fn slide(&self) -> &'a Slide2 {
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
                let content = content.trim_right_matches('\n');
                for ref line in content.split('\n') {
                    view.newline()?;
                    view.present(line)?;

                    let to_fill = cols - line.len();
                    let fill = (0..to_fill).map(|_| ' ').collect::<String>();
                    view.present(&fill)?;
                }
                write!(view, "{}", color::Bg(color::Reset))?;
                write!(view, "{}", color::Fg(color::Reset))?;
            }
            &Element::Quote(ref content) => {
                let cols = view.width() as usize;
                let lines = textwrap::Wrapper::new(cols).wrap(content);
                for ref line in lines {
                    view.newline()?;
                    write!(view, "{}", color::Bg(color::White))?;
                    write!(view, " ")?;
                    write!(view, "{}", color::Bg(color::Reset))?;
                    write!(view, " ")?;
                    view.present(line)?;
                }
                write!(view, "{}", color::Bg(color::Reset))?;
                write!(view, "{}", color::Fg(color::Reset))?;
            }
        }
        Ok(())
    }
}
