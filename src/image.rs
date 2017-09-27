/// To display inline image, we use the following code:
///
/// ```ignore
/// ESC ] 1337 ; File = [optional arguments] : base-64 encoded file contents ^G
/// ```
use base64::encode;

use std::env::var;
use std::fs::File;
use std::path::Path;
use std::io::{Read, Result, Write};

fn support_image() -> bool {
    match var("TERM_PROGRAM") {
        Ok(term) => if term == "iTerm.app" { true } else { false },
        Err(_) => false,
    }
}

fn print_osc<W: Write>(buf: &mut W) -> Result<()> {
    write!(buf, "{}]", '\u{1B}')
}

fn print_st<W: Write>(buf: &mut W) -> Result<()> {
    // char::from(7) is equivalent to \a (not sure why)
    write!(buf, "{}", char::from(7))
}

pub fn inline_image<W: Write, P: AsRef<Path>>(buf: &mut W, path: P) -> Result<()> {
    if !support_image() {
        ::std::process::exit(-1);
    }

    let mut file = File::open(path)?;
    let mut contents = Vec::new();
    file.read_to_end(&mut contents)?;

    print_osc(buf)?;
    write!(buf, "1337;File=")?;
    // print all optional arguments, such as size?
    write!(buf, "inline=1:")?;

    write!(buf, "{}", encode(&contents))?;
    print_st(buf)?;
    write!(buf, "\n")?;
    Ok(())
}
