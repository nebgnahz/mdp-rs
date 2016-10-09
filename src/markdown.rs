use std::string::String;

#[allow(dead_code)]
enum LineType {
    H1,
    H2,
    Paragraph,
    Quote,
    Code,
}

#[allow(dead_code)]
struct Line {
    text: String,
}

#[allow(dead_code)]
struct Slide {
    line: Vec<Line>,
}

pub struct Deck {
    #[allow(dead_code)]
    slides: Vec<Slide>,
}
