/// To display inline image, we use the following code:
///
/// ```ignore
/// ESC ] 1337 ; File = [optional arguments] : base-64 encoded file contents ^G
/// ```
use base64::encode;
use get::get_vec;
use std::collections::HashMap;
use std::env::var;
use std::io::{Error, ErrorKind, Result, Write};
use std::sync::Mutex;

fn support_image() -> bool {
    match var("TERM_PROGRAM") {
        Ok(term) => term == "iTerm.app",
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

lazy_static! {
    static ref IMAGE_STORE: Mutex<HashMap<String, String>> = {
        Mutex::new(HashMap::new())
    };
}

pub fn inline_image<W>(buf: &mut W, name: &str) -> Result<()>
where
    W: Write,
{
    if !support_image() {
        return Err(Error::new(ErrorKind::Other, "inline image not supported"));
    }

    let store = IMAGE_STORE.lock().unwrap();
    let image = match store.get(name) {
        Some(image) => image,
        None => return Err(Error::new(ErrorKind::Other, "image not found")),
    };

    print_osc(buf)?;
    write!(buf, "1337;File=")?;
    write!(buf, "inline=1")?;

    // TODO(benzh) print other optional arguments, such as size?
    write!(buf, ":")?;
    write!(buf, "{}", image)?;
    print_st(buf)?;
    write!(buf, "\n")?;
    Ok(())
}

pub fn retrieve_image(path: String) {
    if let Ok(content) = get_vec(&path) {
        let base64 = encode(&content);
        let mut store = IMAGE_STORE.lock().unwrap();
        println!("inserting {}", path);
        store.insert(path, base64);
    }
}
