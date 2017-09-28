use deck::{Deck, Slide};
use input::ImmediateInput;
use std::borrow::Cow;
use std::io::{Read, Result, Write, stdin};
use std::path::Path;
use termion::{color, cursor};
use termion::event::Key;
use termion::input::TermRead;
use view::View;

fn _show_help(view: &mut View) -> Result<()> {
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

fn file_to_string<P: AsRef<Path>>(p: P) -> Result<String> {
    let mut f = ::std::fs::File::open(p)?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

pub fn play<P: AsRef<Path>>(path: P) -> Result<()> {
    let mut view = View::new()?;
    let input = ImmediateInput::new(0);
    input.set_immediate();
    let mut slide_num = 0;
    loop {
        let content = file_to_string(&path)?;
        let deck = Deck::new(&content)?;
        let ret = show(deck, &mut view, slide_num)?;
        match ret {
            Some(num) => slide_num = num,
            None => break,
        }
    }
    Ok(())
}

fn show(mut deck: Deck, view: &mut View, start: usize) -> Result<Option<usize>> {
    let mut key_reader = stdin().keys();

    deck.goto(start);
    view.clear()?;
    view.present(deck.slide())?;
    show_page_num(&deck, view)?;
    view.hide_cursor()?;
    view.flush()?;

    'outer: loop {
        while let Some(c) = key_reader.next() {
            match c.unwrap() {
                Key::Char('q') => {
                    view.quit()?;
                    return Ok(None);
                }
                Key::Char('s') => {}
                Key::Char('r') => {
                    view.update()?;
                }
                Key::Char('l') => {
                    break 'outer;
                }
                Key::Right | Key::Down | Key::Char('j') | Key::Char(' ') => {
                    deck.next();
                }
                Key::Left | Key::Up | Key::Char('k') => {
                    deck.previous();
                }
                _ => {}
            }

            view.clear()?;
            view.present(deck.slide())?;
            show_page_num(&deck, view)?;
            view.hide_cursor()?;
            view.flush()?;
        }
    }
    Ok(Some(deck.current_num()))
}


fn show_page_num<'a>(deck: &'a Deck, view: &mut View) -> Result<()> {
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
