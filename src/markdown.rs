use std::string::String;

#[allow(dead_code)]
enum LineType {
    H1,
    H2,
    Paragraph,
    Quote,
    Code,
}

pub struct Line {
    pub text: String,
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
    let slide1 = Slide { lines: vec![Line { text: "Hello Slide 1".to_string() }] };
    let slide2 = Slide { lines: vec![Line { text: "Hello Slide 2".to_string() }] };
    Deck {
        slides: vec![slide1, slide2],
        current: 0,
    }
}
