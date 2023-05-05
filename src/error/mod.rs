#[derive(Debug)]
pub enum Error {
    CouldNotDetermineConfigLocation(Vec<String>),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Error::CouldNotDetermineConfigLocation(tried_locations) => {
                write!(
                    f,
                    "Could not determine location for config directory!\nTried:\n"
                )?;
                for loc in tried_locations.iter() {
                    write!(f, "  - {}\n", loc)?;
                }
                Ok(())
            }
        }
    }
}

impl std::error::Error for Error {}
