use Present;
use ViewConfig;
use pulldown_cmark::{Parser, Tag};
use pulldown_cmark::Event::{self, End, Html, InlineHtml, Start, Text};
use pulldown_cmark::Event::{FootnoteReference, HardBreak, SoftBreak};

use split;
use std::borrow::Cow;
use std::io;

#[derive(Default, Debug)]
pub struct Deck<'a> {
    slides: Vec<Slide<'a>>,
    current: usize,
}

#[derive(Default, Debug)]
pub struct Slide<'a> {
    content: Cow<'a, str>,
    offset: usize,
}

impl<'a> Slide<'a> {
    pub fn new((offset, content): (usize, Cow<'a, str>)) -> Self {
        Slide {
            content: content,
            offset: offset,
        }
    }
}

impl<'a> Present for Slide<'a> {
    fn present(&self, view: &mut ViewConfig) -> io::Result<()> {
        let parser = Parser::new(&self.content);
        for element in parser {
            info!("{:?}", element);
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
            &Start(Tag::BlockQuote) => view.start_quote(),
            &End(Tag::BlockQuote) => view.end_quote(),
            &Start(Tag::List(_)) => view.start_list(),
            &End(Tag::List(_)) => view.end_list(),
            &Start(Tag::Item) => view.start_item(),
            &End(Tag::Item) => view.end_item(),
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

impl<'a> Deck<'a> {
    pub fn new(content: &'a str) -> io::Result<Deck<'a>> {
        let slides = split::split(content).map(|s| Slide::new(s)).collect();

        let deck = Deck {
            slides: slides,
            current: 0,
        };

        Ok(deck)
    }

    pub fn add(&mut self, slide: Slide<'a>) {
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

    pub fn slide(&self) -> &'a Slide {
        &self.slides[self.current]
    }

    pub fn current_num(&self) -> usize {
        self.current
    }

    pub fn total_num(&self) -> usize {
        self.slides.len()
    }
}
