use super::{Present, ViewConfig};
use deck::Deck;
use input::ImmediateInput;
use std::io;
use std::io::{stdin, stdout};
use std::io::Write;
use termion;
use termion::event::Key;
use termion::input::TermRead;

fn show_help(view: &mut ViewConfig) -> io::Result<()> {
    write!(
        view,
        "{}{}",
        termion::clear::All,
        termion::cursor::Goto(1, 1)
    )?;
    write!(view, "RMDP: Markdown Presentation in Rust\n")?;
    write!(view, "Press `s` to start")?;
    view.flush()
}

pub fn display(mut deck: Deck) -> io::Result<()> {
    let mut view = ViewConfig::new()?;

    let stdin = stdin();
    let mut stdout = stdout();

    // Modify the terminal behavior to return immediate result (not
    // line-buffered).
    let input = ImmediateInput::new(0);
    input.set_immediate();

    show_help(&mut view)?;
    let mut key_reader = stdin.keys();

    loop {
        while let Some(c) = key_reader.next() {
            match c.unwrap() {
                Key::Char('q') => {
                    return stdout.flush();
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

        try!(write!(
            stdout,
            "{}{}",
            termion::clear::All,
            termion::cursor::Goto(1, 1)
        ));

        {
            let ref slide = deck.slide();
            slide.present(&mut view);

            try!(write!(
                stdout,
                "{}",
                termion::cursor::Goto(1, view.term_height)
            ));
            try!(write!(stdout, "{}", deck.current_num()));
        }

        try!(stdout.flush());
    }
}
