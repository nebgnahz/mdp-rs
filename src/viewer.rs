use std::io;
use std::io::{stdin, stdout};
use std::io::Stdout;
use std::io::Write;
use input::ImmediateInput;
use termion;
use termion::input::TermRead;
use termion::event::Key;
use markdown::Deck;

#[allow(dead_code)]
struct Options {}

fn show_help(stdout: &mut Stdout) -> io::Result<()> {
    write!(*stdout,
           "{}{}RMDP: Markdown Presentation in Rust\n",
           termion::clear::All,
           termion::cursor::Goto(1, 1))
}

pub fn display(mut deck: Deck) -> io::Result<()> {
    let stdin = stdin();
    let mut stdout = stdout();

    // Modify the terminal behavior to return immediate result (not
    // line-buffered).
    let input = ImmediateInput::new(0);
    input.set_immediate();

    let mut key_reader = stdin.keys();
    let (_, h) = try!(termion::terminal_size());

    try!(show_help(&mut stdout));

    loop {
        while let Some(c) = key_reader.next() {
            try!(write!(stdout, " {:?}", c));
            match c.unwrap() {
                Key::Char('s') => {
                }
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

        try!(write!(stdout,
                    "{}{}",
                    termion::clear::All,
                    termion::cursor::Goto(1, 1)));

        {
            let ref slide = deck.slide();
            let ref line = slide.lines[0];
            try!(write!(stdout, "{:?}", line.text));

            try!(write!(stdout, "{}", termion::cursor::Goto(1, h)));
            try!(write!(stdout, "{}", deck.current));
        }

        try!(stdout.flush());
    }
}