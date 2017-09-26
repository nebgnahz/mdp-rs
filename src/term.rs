//! Terminal renderer that takes an iterator of events as input.

use super::deck::{Deck, Element, Slide};

use pulldown_cmark::{Event, Tag};
use std::fmt::Write;
use termion::color;
use termion::style;

struct Ctx<'b, I> {
    iter: I,
    buf: &'b mut String,
    deck: Deck,
    slide: Slide,
}

impl<'a, 'b, I: Iterator<Item = Event<'a>>> Ctx<'b, I> {
    pub fn run(&mut self) -> Result<(), ::std::fmt::Error> {
        while let Some(event) = self.iter.next() {
            match event {
                Event::Start(tag) => self.start_tag(tag)?,
                Event::End(tag) => self.end_tag(tag)?,
                Event::Text(text) => self.buf.push_str(&text.into_owned()),
                Event::Html(_html) |
                Event::InlineHtml(_html) => {
                    unimplemented!{}
                }
                Event::SoftBreak => self.buf.push(' '),
                Event::HardBreak => self.buf.push_str("\n\n"),
                Event::FootnoteReference(_name) => {}
            }
        }

        // push the last slide in
        self.deck.add(self.slide.clone());
        Ok(())
    }

    fn start_tag(&mut self, tag: Tag<'a>) -> Result<(), ::std::fmt::Error> {
        match tag {
            Tag::Paragraph => {}
            Tag::Rule => {}
            Tag::Header(_level) => {}
            Tag::Table(_alignments) => {}
            Tag::TableHead => {}
            Tag::TableRow => {}
            Tag::TableCell => {}
            Tag::BlockQuote => {
                write!(self.buf, "{}", style::Italic)?;
            }
            Tag::CodeBlock(_info) => {
                write!(self.buf, "{}", color::Bg(color::White))?;
                write!(self.buf, "{}", color::Fg(color::Black))?;
            }
            Tag::List(_) => {}
            Tag::Item => {}
            Tag::Emphasis => write!(self.buf, "{}", style::Italic)?,
            Tag::Strong => write!(self.buf, "{}", style::Bold)?,
            Tag::Code => {
                write!(self.buf, "{}", color::Bg(color::White))?;
                write!(self.buf, "{}", color::Fg(color::Black))?;
            }
            Tag::Link(_dest, title) => write!(self.buf, "{}{}", style::Bold, title)?,
            Tag::Image(_dest, title) => write!(self.buf, "{}{}", style::Bold, title)?,
            Tag::FootnoteDefinition(_name) => {}
        }
        Ok(())
    }

    fn end_tag(&mut self, tag: Tag) -> Result<(), ::std::fmt::Error> {
        match tag {
            Tag::Paragraph => {
                // add this element to slide
                self.slide.add(Element::Paragraph(self.buf.clone()));
                self.buf.clear()
            }
            Tag::Rule => {
                self.deck.add(self.slide.clone());
                self.slide.clear();
            }
            Tag::Header(_level) => {
                // write!(self.buf, "{}{}", style::Reset, color::Fg(color::Reset))?;
                self.slide.add(Element::Title(self.buf.clone()));
                self.buf.clear();
            }
            Tag::Table(_alignments) => {}
            Tag::TableHead => {}
            Tag::TableRow => {}
            Tag::TableCell => {}
            Tag::BlockQuote => {
                write!(self.buf, "{}", style::Reset)?;
                self.slide.add(Element::Quote(self.buf.clone()));
                self.buf.clear();
            }
            Tag::CodeBlock(_info) => {
                write!(self.buf, "{}", color::Fg(color::Black))?;
                write!(self.buf, "{}", color::Bg(color::Reset))?;
            }
            Tag::List(_) => {}
            Tag::Item => {}
            Tag::Emphasis => write!(self.buf, "{}", style::Reset)?,
            Tag::Strong => write!(self.buf, "{}", style::Reset)?,
            Tag::Code => {
                write!(self.buf, "{}", color::Fg(color::Reset))?;
                write!(self.buf, "{}", color::Bg(color::Reset))?;
                self.slide.add(Element::Code(self.buf.clone()));
                self.buf.clear();
            }
            Tag::Link(_dest, _title) => write!(self.buf, "{}", style::Reset)?,
            Tag::Image(_dest, _title) => write!(self.buf, "{}", style::Reset)?,
            Tag::FootnoteDefinition(_name) => {}
        }
        Ok(())
    }
}

pub fn terminalize<'a, I>(buf: &mut String, iter: I) -> Deck
where
    I: Iterator<Item = Event<'a>>,
{
    let mut ctx = Ctx {
        iter: iter,
        buf: buf,
        deck: Deck::default(),
        slide: Slide::default(),
    };
    ctx.run().unwrap();
    ctx.deck
}
