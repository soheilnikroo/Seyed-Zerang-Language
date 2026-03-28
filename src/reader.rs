use std::{fs::read_to_string, io};

pub struct Source {
    pub contents: String,
}

impl Source {
    pub fn from(s: impl Into<String>) -> Source {
        Source { contents: s.into() }
    }
}

#[derive(Debug)]
pub struct Error {
    pub msg: String,
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error {
            msg: format!("{err}"),
        }
    }
}

pub fn read_source(filename: &str) -> Result<Source, Error> {
    println!("Reading Source.");
    let contents = read_to_string(filename)?;
    Ok(Source { contents })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn its_alive() {
        assert_eq!(true, true)
    }
}
