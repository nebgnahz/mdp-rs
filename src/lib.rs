#![feature(plugin)]
#![plugin(clippy)]

#[macro_use]
extern crate lazy_static;
extern crate base64;
#[macro_use]
extern crate log;
extern crate termion;
extern crate termios;
extern crate pulldown_cmark;
extern crate reqwest;

mod deck;
mod input;
mod present;
mod split;
mod view;
mod viewer;
mod image;
mod get;

pub use deck::Deck;
pub use image::inline_image;
pub use present::Present;
pub use viewer::play;
