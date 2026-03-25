/// Read source code from file

pub type Source = ();

pub fn read_source(filename: &str) -> Source {
    println!("Reading Source.");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn its_alive() {
        assert_eq!(true, true)
    }
}
