#![cfg_attr(feature="clippy", feature(plugin))]

extern crate env_logger;
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
    env_logger::init().unwrap();
    let opt = Opt::from_args();
    run(opt).unwrap();
}

fn run(opt: Opt) -> Result<()> {
    mdp::play(&opt.file)
}
