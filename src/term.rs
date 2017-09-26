//! HTML renderer that takes an iterator of events as input.
use std::fmt::Write;
use termion::color;
use termion::style;
use super::deck::{Slide, Deck};

use pulldown_cmark::{Event, Tag};

struct Ctx<'b, I> {
    iter: I,
    buf: &'b mut String,
    deck: Deck,
    slide: Slide,
}

impl<'a, 'b, I: Iterator<Item = Event<'a>>> Ctx<'b, I> {
    fn fresh_line(&mut self) {
        if !(self.buf.is_empty() || self.buf.ends_with('\n')) {
            self.buf.push('\n');
        }
    }

    pub fn run(&mut self) -> Result<(), ::std::fmt::Error> {
        while let Some(event) = self.iter.next() {
            match event {
                Event::Start(tag) => self.start_tag(tag)?,
                Event::End(tag) => self.end_tag(tag)?,
                Event::Text(text) => self.buf.push_str(&text.into_owned()),
                Event::Html(html) |
                Event::InlineHtml(html) => self.buf.push_str(&html),
                Event::SoftBreak => self.buf.push('\n'),
                Event::HardBreak => self.buf.push_str("\n\n\n"),
                Event::FootnoteReference(_name) => {}
            }
        }
        Ok(())
    }

    fn start_tag(&mut self, tag: Tag<'a>) -> Result<(), ::std::fmt::Error> {
        match tag {
            Tag::Paragraph => {}
            Tag::Rule => {
                self.fresh_line();
                self.buf.push_str("<hr />\n")
            }
            Tag::Header(_level) => {
                write!(self.buf, "{}{}", color::Fg(color::Red), style::Bold)?;
            }
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
            Tag::Paragraph => {}
            Tag::Rule => {
                self.deck.add(self.slide.clone());
                self.slide.clear();
            }
            Tag::Header(_level) => {
                write!(self.buf, "{}{}", style::Reset, color::Fg(color::Reset))?;
            }
            Tag::Table(_alignments) => {}
            Tag::TableHead => {}
            Tag::TableRow => {}
            Tag::TableCell => {}
            Tag::BlockQuote => {
                write!(self.buf, "{}", style::Reset)?;
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
            }
            Tag::Link(_dest, _title) => write!(self.buf, "{}", style::Reset)?,
            Tag::Image(_dest, _title) => write!(self.buf, "{}", style::Reset)?,
            Tag::FootnoteDefinition(_name) => {}
        }
        Ok(())
    }
}

/// Iterate over an `Iterator` of `Event`s, generate HTML for each `Event`, and
/// push it to a `String`.
///
/// # Examples
///
/// ```
/// use pulldown_cmark::{html, Parser};
///
/// let markdown_str = r#"
/// hello
/// =====
///
/// * alpha
/// * beta
/// "#;
/// let parser = Parser::new(markdown_str);
///
/// let mut html_buf = String::new();
/// html::push_html(&mut html_buf, parser);
///
/// assert_eq!(html_buf, r#"<h1>hello</h1>
/// <ul>
/// <li>alpha</li>
/// <li>beta</li>
/// </ul>
/// "#);
/// ```
pub fn terminalize<'a, I: Iterator<Item = Event<'a>>>(buf: &mut String, iter: I) -> Deck {
    let mut ctx = Ctx {
        iter: iter,
        buf: buf,
        deck: Deck::default(),
        slide: Slide::default(),
    };
    ctx.run().unwrap();
    ctx.deck
}
