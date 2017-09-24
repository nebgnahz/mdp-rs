extern crate mdp;
extern crate structopt;
#[macro_use]
extern crate structopt_derive;

use std::fs::File;
use std::io::Read;
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
    let mut f = try!(File::open(opt.file));
    let mut s = String::new();
    try!(f.read_to_string(&mut s));
    println!("{}", s);

    // let deck = mdp::deck::demo();
    // let deck = mdp::markdown::parse_document(&s);
    // mdp::viewer::display(deck).unwrap();
    Ok(())
}
