use reqwest;
use std::fs::File;
use std::io::{Error, ErrorKind, Read, Result};

pub fn get_vec(path: &str) -> Result<Vec<u8>> {
    let mut contents = Vec::new();
    match reqwest::Url::parse(path) {
        Ok(url) => {
            reqwest::get(url)
                .map_err(|_e| Error::new(ErrorKind::NotConnected, "reqwest"))
                .and_then(|mut r| r.read_to_end(&mut contents))?;
        }
        Err(_) => {
            File::open(&path).and_then(
                |mut r| r.read_to_end(&mut contents),
            )?;
        }
    }
    Ok(contents)
}

pub fn get_string(path: &str) -> Result<String> {
    let mut contents = String::new();
    match reqwest::Url::parse(path) {
        Ok(url) => {
            reqwest::get(url)
                .map_err(|_e| Error::new(ErrorKind::NotConnected, "reqwest"))
                .and_then(|mut r| r.read_to_string(&mut contents))?;
        }
        Err(_) => {
            File::open(&path).and_then(
                |mut r| r.read_to_string(&mut contents),
            )?;
        }
    }
    Ok(contents)
}
