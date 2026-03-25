/// Read source code from file

pub type Source = ();
pub type Error = ();

pub fn read_source(filename: &str) -> Result<Source, Error> {
    println!("Reading Source.");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn its_alive() {
        assert_eq!(true, true)
    }
}
