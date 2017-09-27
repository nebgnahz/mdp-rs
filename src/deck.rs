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

    pub fn content(&self) -> Cow<'a, str> {
        self.content.clone()
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
