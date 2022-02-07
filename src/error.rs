use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct StructMapError(String);

impl StructMapError {
    pub fn new<S: Into<String>>(content: S) -> Self {
        StructMapError(content.into())
    }
}

impl fmt::Display for StructMapError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "structmap error: {}", self.0)
    }
}

impl Error for StructMapError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(self.0)
    }
}
