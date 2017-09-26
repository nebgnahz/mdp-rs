extern crate mdp;
extern crate structopt;
#[macro_use]
extern crate structopt_derive;

use std::io::{Read, Result};
use std::string::String;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "mdp", about = "A markdown presentation tool in Rust.")]
struct Opt {
    /// Path to the markdown file.
    #[structopt(help = "Markdown file")]
    file: String,
}

fn main() {
    let opt = Opt::from_args();
    run(opt).unwrap();
}

fn run(opt: Opt) -> Result<()> {
    let mut f = ::std::fs::File::open(opt.file)?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;

    let deck = mdp::Deck2::new(&s)?;
    // println!("{:?}", deck);
    mdp::display(deck).unwrap();

    // let deck = mdp::deck::demo();
    // let deck = mdp::markdown::parse_document(&s);
    // mdp::viewer::display(deck).unwrap();
    Ok(())
}
