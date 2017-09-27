use input::ImmediateInput;
use super::ViewConfig;
use deck::{Deck, Slide};
use std::borrow::Cow;
use std::io;
use std::io::Write;
use std::io::stdin;
use termion::{color, cursor};
use termion::event::Key;
use termion::input::TermRead;
use chan_signal::{self, Signal};
use chan;

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
    let signals = [Signal::INT, Signal::TSTP, Signal::TERM, Signal::CONT];
    let signal = chan_signal::notify(&signals);

    let input = ImmediateInput::new(0);
    input.set_immediate();

    show_help(&mut view)?;

    let (key_tx, key) = chan::sync(8);
    ::std::thread::spawn(move || loop {
        while let Some(c) = key_reader.next() {
            if c.is_ok() {
                key_tx.send(c.unwrap())
            }
        }
    });

    // Wait for a signal or for work to be done.
    let mut counter = 1;
    loop {
        chan_select! {
            signal.recv() -> signal => {
                let signal = signal.unwrap();
                println!("{:?}x{}", signal, counter);
                counter +=1;
                match signal {
                    Signal::INT | Signal::TERM => {
                        view.quit()?;
                        return Ok(());
                    }
                    Signal::TSTP => {
                        view.quit()?;
                        println!("{:?}", signal);
                        chan_signal::kill_this(Signal::STOP);
                    }
                    Signal::CONT => {
                        input.set_immediate();
                        view.clear()?;
                        view.present(deck.slide())?;
                        show_page_num(&deck, &mut view)?;
                        view.hide_cursor()?;
                        view.flush()?;
                    }
                    _ => {
                        unreachable!{}
                    }
                }
            },
            key.recv() -> key => {
                let key = key.unwrap();
                match key {
                    Key::Char('q') => {
                        return view.quit();
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

                view.clear()?;
                view.present(deck.slide())?;
                show_page_num(&deck, &mut view)?;
                view.hide_cursor()?;
                view.flush()?;
            },
        }
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
