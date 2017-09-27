use super::ViewConfig;
use deck::{Deck, Slide};
use input::ImmediateInput;
use std::borrow::Cow;
use std::io;
use std::io::Write;
use std::io::stdin;
use termion::{color, cursor};
use termion::event::Key;
use termion::input::TermRead;

fn show_help(view: &mut ViewConfig) -> io::Result<()> {
    let help = r#"
# mdp: a markdown presentation tool built in Rust

- j, left, up => next slide
- k, right, down, space => previous slide
- a - go to first slide
- $ - go to last slide
- 1-9 - go to slide n
- r - reload input file
- q - exit

"#;

    let slide = Slide::new((0, Cow::from(&help[..])));
    view.present(&slide)?;
    view.info()
}

pub fn display(mut deck: Deck) -> io::Result<()> {
    let mut view = ViewConfig::new()?;
    let mut key_reader = stdin().keys();

    // Modify the terminal behavior to return immediate result (not
    // line-buffered).
    let input = ImmediateInput::new(0);
    input.set_immediate();

    show_help(&mut view)?;

    loop {
        while let Some(c) = key_reader.next() {
            match c.unwrap() {
                Key::Char('q') => {
                    view.reset()?;
                    view.show_cursor()?;
                    return view.flush();
                }
                Key::Char('s') => {}
                Key::Char('r') => {
                    view.update()?;
                }
                Key::Right | Key::Down | Key::Char('j') | Key::Char(' ') => {
                    deck.next();
                }
                Key::Left | Key::Up | Key::Char('k') => {
                    deck.previous();
                }
                _ => {}
            }
            break;
        }

        view.clear()?;
        view.present(deck.slide())?;
        show_page_num(&deck, &mut view)?;
        view.hide_cursor()?;
        view.flush()?;
    }
}

fn show_page_num<'a>(deck: &'a Deck, view: &mut ViewConfig) -> io::Result<()> {
    use std::fmt::Write;
    let mut s = String::new();
    write!(&mut s, "{} / {}", deck.current_num() + 1, deck.total_num()).unwrap();
    let (mut x, y) = view.right_bottom();
    x = x - s.len() as u16;
    write!(view, "{}", cursor::Goto(x, y))?;
    write!(view, "{}", color::Fg(color::LightRed))?;
    write!(view, "{}", s)?;
    write!(view, "{}", color::Fg(color::Reset))?;
    Ok(())
}
