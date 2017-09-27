extern crate env_logger;
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
    env_logger::init().unwrap();
    let opt = Opt::from_args();
    run(opt).unwrap();
}

fn run(opt: Opt) -> Result<()> {
    let mut f = ::std::fs::File::open(opt.file)?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;

    let deck = mdp::Deck::new(&s)?;
    mdp::display(deck).unwrap();
    Ok(())
}
