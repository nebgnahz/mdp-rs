use deck::Slide;
use pulldown_cmark::{Parser, Tag};
use pulldown_cmark::Event::{self, End, Html, InlineHtml, Start, Text};
use pulldown_cmark::Event::{FootnoteReference, HardBreak, SoftBreak};

use std::io::Result;
use view::View;

pub trait Present {
    fn present(&self, view: &mut View) -> Result<()>;
}

impl<'a> Present for Slide<'a> {
    fn present(&self, view: &mut View) -> Result<()> {
        let content = self.content();
        let parser = Parser::new(&content);
        for element in parser {
            info!("{:?}", element);
        }

        let parser = Parser::new(&content);
        for element in parser {
            view.present(&element)?;
        }
        Ok(())
    }
}

impl<'a> Present for Event<'a> {
    fn present(&self, view: &mut View) -> Result<()> {
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
            &HardBreak => {
                view.newline()?;
                view.newline()
            }
        }
    }
}
