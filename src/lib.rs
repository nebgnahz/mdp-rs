#[macro_use]
extern crate log;
extern crate termion;
extern crate termios;
extern crate pulldown_cmark;

mod deck;
mod input;
mod present;
mod split;
mod view;
mod viewer;

pub use present::Present;
pub use deck::Deck;
pub use viewer::display;
