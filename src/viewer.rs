use super::ViewConfig;
use deck::Deck2;
use input::ImmediateInput;
use std::io;
use std::io::Write;
use std::io::stdin;
use termion;
use termion::event::Key;
use termion::input::TermRead;

fn show_help(view: &mut ViewConfig) -> io::Result<()> {
    view.clear()?;
    write!(view, "RMDP: Markdown Presentation in Rust")?;
    view.newline()?;
    write!(view, "Press `s` to start")?;
    view.newline()?;
    view.info()?;
    view.flush()
}

pub fn display(mut deck: Deck2) -> io::Result<()> {
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
                    return view.flush();
                }
                Key::Char('s') => {}
                Key::Down | Key::Char('j') => {
                    deck.next();
                }
                Key::Up | Key::Char('k') => {
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

fn show_page_num<'a>(deck: &'a Deck2, view: &mut ViewConfig) -> io::Result<()> {
    use std::fmt::Write;
    let mut s = String::new();
    write!(&mut s, "{}/{}", deck.current_num() + 1, deck.total_num()).unwrap();
    let (mut x, y) = view.right_bottom();
    x = x - s.len() as u16;
    write!(view, "{}", termion::cursor::Goto(x, y))?;
    write!(view, "{}", s)?;
    Ok(())
}
