use image::inline_image;
use present::Present;
use std::borrow::Cow;
use std::cmp::min;
use std::io::{Result, Stdout, Write, stdout};
use termion::{self, color, cursor, style};

#[derive(Debug)]
pub struct View {
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

    /// Whether we have the image successfully loaded or not. This affects
    /// whether or not we show image titie.
    Image(bool),

    /// List
    List(usize, ListState),
}

#[derive(Debug, Clone, Copy)]
enum ListState {
    ParagraphFirst,
    ItemContinue,
    JustEnd,
}

impl View {
    pub fn new() -> Result<Self> {
        let (term_width, term_height) = termion::terminal_size()?;
        let width = min(80, term_width - 4);
        let bottom_margin = term_height / 10;

        let view = View {
            term_width: term_width,
            term_height: term_height,
            stdout: stdout(),

            width: width,

            left_margin: (term_width - width) / 2 - 1,
            right_margin: (term_width - width) / 2,
            top_margin: 2,
            bottom_margin: bottom_margin,

            ctx: Context::Default,
        };
        Ok(view)
    }

    pub fn update(&mut self) -> Result<()> {
        let (term_width, term_height) = termion::terminal_size()?;
        let width = min(80, term_width - 4);

        self.term_width = term_width;
        self.term_height = term_height;
        self.width = width;
        self.left_margin = (term_width - width) / 2 - 1;
        self.right_margin = (term_width - width) / 2;
        self.bottom_margin = term_height / 10;
        Ok(())
    }

    pub fn clear(&mut self) -> Result<()> {
        write!(
            self.stdout,
            "{}{}",
            termion::clear::All,
            cursor::Goto(self.left_margin, self.top_margin)
        )
    }

    pub fn quit(&mut self) -> Result<()> {
        self.reset()?;
        self.show_cursor()?;
        self.flush()
    }

    pub fn reset(&mut self) -> Result<()> {
        write!(self.stdout, "{}{}", termion::clear::All, cursor::Goto(1, 1))
    }

    pub fn info(&mut self) -> Result<()> {
        write!(self.stdout, "Your terminal is ")?;
        write!(self.stdout, "{}x{}", self.term_width, self.term_height)
    }

    pub fn newline(&mut self) -> Result<()> {
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

    pub fn present<P: Present>(&mut self, p: &P) -> Result<()> {
        p.present(self)
    }

    pub fn show_text<'a>(&mut self, text: &Cow<'a, str>) -> Result<()> {
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
            Context::List(i, list_state) => {
                match list_state {
                    ListState::ParagraphFirst => {
                        self.ctx = Context::List(i, ListState::ItemContinue);
                    }
                    ListState::ItemContinue => {
                        write!(self, "   ")?;
                        (0..i).map(|_| write!(self, "   ")).count();
                    }
                    ListState::JustEnd => {}
                }
                self.present(text)
            }
            Context::Image(true) => Ok(()),
            Context::Image(false) => self.present(text),
        }
    }

    pub fn hide_cursor(&mut self) -> Result<()> {
        write!(self, "{}", cursor::Hide)
    }

    pub fn show_cursor(&mut self) -> Result<()> {
        write!(self, "{}", cursor::Show)
    }

    pub fn start_code(&mut self) -> Result<()> {
        write!(self, "{}", color::Bg(color::LightWhite))?;
        write!(self, "{}", color::Fg(color::Black))
    }

    pub fn end_code(&mut self) -> Result<()> {
        write!(self, "{}", color::Fg(color::Reset))?;
        write!(self, "{}", color::Bg(color::Reset))
    }

    pub fn start_codeblock(&mut self) -> Result<()> {
        self.ctx = Context::CodeBlock(0);
        self.newline()?;
        write!(self, "{}", color::Bg(color::LightWhite))?;
        write!(self, "{}", color::Fg(color::Black))
    }

    pub fn end_codeblock(&mut self) -> Result<()> {
        self.ctx = Context::Default;
        self.newline()?;
        write!(self, "{}", color::Fg(color::Reset))?;
        write!(self, "{}", color::Bg(color::Reset))
    }

    pub fn start_italic(&mut self) -> Result<()> {
        write!(self, "{}", style::Italic)
    }

    pub fn end_italic(&mut self) -> Result<()> {
        write!(self, "{}", style::NoItalic)
    }

    pub fn start_bold(&mut self) -> Result<()> {
        write!(self, "{}", style::Bold)
    }

    pub fn end_bold(&mut self) -> Result<()> {
        write!(self, "{}", style::Reset)
    }

    pub fn start_paragraph(&mut self) -> Result<()> {
        match self.ctx {
            Context::List(_, ListState::ParagraphFirst) => Ok(()),
            _ => self.newline(),
        }
    }

    pub fn end_paragraph(&mut self) -> Result<()> {
        self.newline()
    }

    pub fn start_quote(&mut self) -> Result<()> {
        self.ctx = Context::Quote;
        self.newline()
    }

    pub fn end_quote(&mut self) -> Result<()> {
        self.ctx = Context::Default;
        self.newline()
    }

    pub fn start_list(&mut self) -> Result<()> {
        match self.ctx {
            Context::Default => {
                self.ctx = Context::List(0, ListState::ParagraphFirst);
            }
            Context::List(i, _) => {
                self.ctx = Context::List(i + 1, ListState::ParagraphFirst);
            }
            _ => unreachable!{},
        }
        self.newline()
    }

    pub fn end_list(&mut self) -> Result<()> {
        match self.ctx {
            Context::List(0, _) => {
                self.ctx = Context::Default;
            }
            Context::List(i, state) => {
                self.ctx = Context::List(i - 1, state);
            }
            ref ctx => {
                error!("{:?}", &ctx);
                unreachable!{}
            }
        }
        self.newline()
    }

    pub fn start_item(&mut self) -> Result<()> {
        match self.ctx {
            Context::List(i, _) => {
                (0..i).map(|_| write!(self, "   ")).count();
                write!(self, "+- ")?;
                self.ctx = Context::List(i, ListState::ParagraphFirst);
            }
            _ => unimplemented!{},
        }
        Ok(())
    }

    pub fn end_item(&mut self) -> Result<()> {
        match self.ctx {
            Context::List(_level, ListState::JustEnd) => Ok(()),
            Context::List(level, _) => {
                self.ctx = Context::List(level, ListState::JustEnd);
                self.newline()
            }
            _ => {
                unreachable!{}
            }
        }
    }

    pub fn start_header(&mut self, _level: i32) -> Result<()> {
        self.newline()?;
        write!(self, "{}{}", color::Fg(color::LightCyan), style::Underline)
    }

    pub fn end_header(&mut self, _level: i32) -> Result<()> {
        write!(
            self,
            "{}{}",
            style::NoUnderline,
            color::Fg(color::Reset),
        )?;
        self.newline()
    }

    pub fn start_image<'a>(&mut self, path: &Cow<'a, str>) -> Result<()> {
        let path = path.clone().into_owned();
        match inline_image(self, path) {
            Ok(()) => self.ctx = Context::Image(true),
            Err(_) => self.ctx = Context::Image(false),
        }
        self.newline()
    }

    pub fn end_image<'a>(&mut self, _path: &Cow<'a, str>) -> Result<()> {
        self.ctx = Context::Default;
        Ok(())
    }
}

impl Write for View {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        self.stdout.write(buf)
    }
    fn flush(&mut self) -> Result<()> {
        self.stdout.flush()
    }
}

impl Present for String {
    fn present(&self, view: &mut View) -> Result<()> {
        write!(view, "{}", self)
    }
}

impl<'a> Present for &'a str {
    fn present(&self, view: &mut View) -> Result<()> {
        write!(view, "{}", self)
    }
}

impl<'a> Present for Cow<'a, str> {
    fn present(&self, view: &mut View) -> Result<()> {
        write!(view, "{}", self)
    }
}
