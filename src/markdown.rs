use pulldown_cmark::{Parser, Event, Tag};
use deck::{Element, Line, Deck, Slide};

pub fn parse_document(text: &str) -> Deck {
    let mut opt_element: Option<Element> = None;
    let mut content: String = String::new();
    let mut slide = Slide::default();
    let mut deck = Deck::default();
    for event in Parser::new(text) {
        match event {
            Event::Start(tag) => opt_element = Some(match_tag_to_element(&tag)),
            Event::Text(text) => content.push_str(&text),
            Event::End(tag) => {
                if let Some(elem) = opt_element {
                    if match_tag_to_element(&tag) == elem {
                        if elem == Element::Rule {
                            deck.add_slide(slide.clone());
                            slide.clear();
                        } else {
                            slide.add_line(Line {
                                text: content.clone(),
                                elem: elem,
                            });
                            content.clear()
                        }
                    }
                }
            }
            _ => {}
        }
    }
    deck.add_slide(slide);
    deck
}

fn match_tag_to_element(tag: &Tag) -> Element {
    match *tag {
        Tag::Paragraph => Element::Paragraph,
        Tag::Header(_) => Element::H1,
        Tag::BlockQuote => Element::Quote,
        Tag::CodeBlock(ref _lang) => Element::Code,
        Tag::Rule => Element::Rule,
        _ => Element::Code,
    }
}
