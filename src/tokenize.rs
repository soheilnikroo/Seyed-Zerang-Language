use crate::reader::Source;

pub type Tokens = ();

pub fn tokenize(source: Source) -> Tokens {
    println!("Tokenize the code");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn its_alive() {
        assert_eq!(true, true)
    }
}
