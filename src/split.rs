//! Split a full markdown file into each slides

use image::retrieve_image;
use pulldown_cmark::{Event, Parser, Tag};
use std::borrow::Cow;

pub struct Split<'a> {
    buf: &'a str,
    parser: Parser<'a>,
    start_offset: usize,
    end_offset: usize,

    first_page: bool,
}

impl<'a> Split<'a> {}

impl<'a> Iterator for Split<'a> {
    type Item = (usize, Cow<'a, str>);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let event = match self.parser.next() {
                Some(event) => event,
                None => {
                    if self.start_offset == self.buf.len() {
                        return None;
                    } else {
                        let end = self.buf.len();
                        let content = Cow::from(&self.buf[self.start_offset..end]);
                        let ret = (self.start_offset, content);
                        self.start_offset = end;
                        return Some(ret);
                    }
                }
            };
            trace!("{:?}", event);
            match event {
                Event::Start(Tag::Rule) => {
                    if let Some(Event::End(Tag::Rule)) = self.parser.next() {
                        let s = &self.buf[self.start_offset..self.end_offset];
                        let content = Cow::from(s);
                        let ret = (self.start_offset, content);
                        self.start_offset = self.parser.get_offset();

                        // One page ready
                        self.first_page = false;
                        return Some(ret);
                    } else {
                        // Tag mismatch, error
                        error!("Surprising markdown file/parser for Tag::Rule");
                        ::std::process::exit(-1);
                    }
                }
                Event::Start(Tag::Image(path, _)) => {
                    let path = String::from(path);
                    if self.first_page {
                        // synchronous read for first page
                        retrieve_image(path);
                    } else {
                        // asynchronous read for the rest of the slides
                        ::std::thread::spawn(move || retrieve_image(path));
                    }
                }
                _ => {}
            }
            self.end_offset = self.parser.get_offset();
        }
    }
}

pub fn split<'a>(buf: &'a str) -> Split<'a> {
    Split {
        buf: buf,
        parser: Parser::new(buf),
        start_offset: 0,
        end_offset: 0,
        first_page: true,
    }
}
