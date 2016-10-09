extern crate rmdp;
extern crate getopts;
use std::string::String;
use std::env;
use getopts::Options;

fn print_usage(opts: Options) {
    let brief = format!("Usage: rmdp [OPTION]... [FILE]");
    print!("{}", opts.usage(&brief));
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut opts = Options::new();
    opts.optflag("h", "help", "print the usage");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(e) => panic!(e.to_string()),
    };
    if matches.opt_present("h") {
        print_usage(opts);
        return;
    }

    let deck = rmdp::markdown::demo();
    rmdp::viewer::display(deck).unwrap();
}
