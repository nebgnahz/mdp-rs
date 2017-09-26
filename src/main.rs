extern crate mdp;
extern crate structopt;
#[macro_use]
extern crate structopt_derive;

use std::io::Result;
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
    let deck = mdp::Deck::from_path(opt.file)?;
    // println!("{:?}", deck);
    mdp::display(deck).unwrap();
    // let deck = mdp::deck::demo();
    // let deck = mdp::markdown::parse_document(&s);
    // mdp::viewer::display(deck).unwrap();
    Ok(())
}
