extern crate mdp;
extern crate getopts;
use std::io::Read;
use std::io::Result;
use std::fs::File;
use std::string::String;
use std::env;
use getopts::Options;

fn print_usage(opts: Options) {
    let brief = format!("Usage: mdp [OPTION]... [FILE]");
    print!("{}", opts.usage(&brief));
}

fn main() {
    run().unwrap();
}

fn run() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let mut opts = Options::new();
    opts.optflag("h", "help", "print the usage");
    opts.reqopt("f", "file", "markdown file to present", "presen.md");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(_) => panic!("invalid argument"),
    };
    if matches.opt_present("h") {
        print_usage(opts);
        return Ok(());
    }
    let filename = matches.opt_str("f").expect("You need to provide a file to present");
    let mut f = try!(File::open(filename));
    let mut s = String::new();
    try!(f.read_to_string(&mut s));
    println!("{}", s);
    // let deck = mdp::deck::demo();
    let deck = mdp::markdown::parse_document(&s);
    mdp::viewer::display(deck).unwrap();
    Ok(())
}
