#[macro_use]
extern crate log;
extern crate termion;
extern crate termios;
extern crate pulldown_cmark;

mod deck;
//mod term;

pub use deck::Deck;
pub use viewer::display;
mod viewer;
mod input;
mod split;

use std::borrow::Cow;
use std::io::{self, Stdout, Write};
use termion::{color, cursor, style};

// mod style;
// pub mod markdown;

trait Present {
    fn present(&self, view: &mut ViewConfig) -> io::Result<()>;
}

#[derive(Debug)]
struct ViewConfig {
    term_width: u16,
    term_height: u16,
    stdout: Stdout,

    width: u16,
    left_margin: u16,
    right_margin: u16,
    top_margin: u16,
    bottom_margin: u16,

    ctx: Context,
}

#[derive(Debug)]
enum Context {
    Default,
    Quote,
    _Paragraph,
    CodeBlock(usize),
}

impl ViewConfig {
    pub fn new() -> io::Result<Self> {
        let (term_width, term_height) = termion::terminal_size()?;
        let width = std::cmp::min(80, term_width - 4);

        let view = ViewConfig {
            term_width: term_width,
            term_height: term_height,
            stdout: io::stdout(),

            width: width,

            left_margin: (term_width - width) / 2 - 1,
            right_margin: (term_width - width) / 2,
            top_margin: 2,
            bottom_margin: 10,

            ctx: Context::Default,
        };
        Ok(view)
    }

    pub fn clear(&mut self) -> io::Result<()> {
        write!(
            self.stdout,
            "{}{}",
            termion::clear::All,
            cursor::Goto(self.left_margin, self.top_margin)
        )
    }

    pub fn reset(&mut self) -> io::Result<()> {
        write!(self.stdout, "{}{}", termion::clear::All, cursor::Goto(1, 1))
    }

    pub fn info(&mut self) -> io::Result<()> {
        write!(self.stdout, "Your terminal is ")?;
        write!(self.stdout, "{}x{}", self.term_width, self.term_height)
    }

    pub fn newline(&mut self) -> io::Result<()> {
        write!(self.stdout, "\n{}", cursor::Right(self.left_margin - 1))
    }

    pub fn right_bottom(&self) -> (u16, u16) {
        let bottom = self.term_height - self.bottom_margin;
        let right = self.term_width - self.right_margin;
        (right, bottom)
    }

    pub fn width(&self) -> u16 {
        self.width
    }

    pub fn present<P: Present>(&mut self, p: &P) -> io::Result<()> {
        p.present(self)
    }

    pub fn show_text<'a>(&mut self, text: &Cow<'a, str>) -> io::Result<()> {
        match self.ctx {
            Context::Default => self.present(text),
            Context::_Paragraph => self.present(text),
            Context::CodeBlock(i) => {
                if text.ends_with("\n") {
                    let content = text.trim_right_matches('\n');

                    let cols = self.width() as usize;
                    let to_fill = cols - content.len() - i;
                    let fill = (0..to_fill).map(|_| ' ').collect::<String>();

                    self.present(&content)?;
                    self.present(&fill)?;
                    self.newline()?;
                    self.ctx = Context::CodeBlock(0);
                } else {
                    self.present(text)?;
                    self.ctx = Context::CodeBlock(i + text.len());
                }
                Ok(())
            }
            Context::Quote => {
                write!(
                    self,
                    "{} {} ",
                    color::Bg(color::LightWhite),
                    color::Bg(color::Reset)
                )?;
                self.present(text)
            }
        }
    }

    pub fn hide_cursor(&mut self) -> io::Result<()> {
        write!(self, "{}", cursor::Hide)
    }

    pub fn show_cursor(&mut self) -> io::Result<()> {
        write!(self, "{}", cursor::Show)
    }

    pub fn start_code(&mut self) -> io::Result<()> {
        write!(self, "{}", color::Bg(color::LightWhite))?;
        write!(self, "{}", color::Fg(color::Black))
    }

    pub fn end_code(&mut self) -> io::Result<()> {
        write!(self, "{}", color::Fg(color::Reset))?;
        write!(self, "{}", color::Bg(color::Reset))
    }

    pub fn start_codeblock(&mut self) -> io::Result<()> {
        self.ctx = Context::CodeBlock(0);
        write!(self, "{}", color::Bg(color::LightWhite))?;
        write!(self, "{}", color::Fg(color::Black))
    }

    pub fn end_codeblock(&mut self) -> io::Result<()> {
        self.ctx = Context::Default;
        self.newline()?;
        write!(self, "{}", color::Fg(color::Reset))?;
        write!(self, "{}", color::Bg(color::Reset))
    }

    pub fn start_italic(&mut self) -> io::Result<()> {
        write!(self, "{}", style::Italic)
    }

    pub fn end_italic(&mut self) -> io::Result<()> {
        write!(self, "{}", style::NoItalic)
    }

    pub fn start_bold(&mut self) -> io::Result<()> {
        write!(self, "{}", style::Bold)
    }

    pub fn end_bold(&mut self) -> io::Result<()> {
        write!(self, "{}", style::Reset)
    }

    pub fn start_paragraph(&mut self) -> io::Result<()> {
        self.newline()
    }

    pub fn end_paragraph(&mut self) -> io::Result<()> {
        self.newline()
    }

    pub fn start_quote(&mut self) -> io::Result<()> {
        self.ctx = Context::Quote;
        self.newline()
    }

    pub fn end_quote(&mut self) -> io::Result<()> {
        self.ctx = Context::Default;
        self.newline()
    }

    pub fn start_header(&mut self, _level: i32) -> io::Result<()> {
        self.newline()?;
        write!(self, "{}{}", color::Fg(color::LightCyan), style::Underline)
    }

    pub fn end_header(&mut self, _level: i32) -> io::Result<()> {
        write!(
            self,
            "{}{}",
            style::NoUnderline,
            color::Fg(color::Reset),
        )?;
        self.newline()
    }
}

impl Write for ViewConfig {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.stdout.write(buf)
    }
    fn flush(&mut self) -> io::Result<()> {
        self.stdout.flush()
    }
}

impl Present for String {
    fn present(&self, view: &mut ViewConfig) -> io::Result<()> {
        write!(view, "{}", self)
    }
}

impl<'a> Present for &'a str {
    fn present(&self, view: &mut ViewConfig) -> io::Result<()> {
        write!(view, "{}", self)
    }
}

impl<'a> Present for Cow<'a, str> {
    fn present(&self, view: &mut ViewConfig) -> io::Result<()> {
        write!(view, "{}", self)
    }
}
