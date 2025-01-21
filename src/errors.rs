/*!
Error handling for `text_formatter`.
*/

use std::error::Error;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::io;

#[derive(Debug)]
pub enum FormatterError {
    IoError(io::Error),
    ParseError(String), // Could be from JSON/TOML or text parsing
}

impl Display for FormatterError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            FormatterError::IoError(e) => write!(f, "IO Error: {}", e),
            FormatterError::ParseError(e) => write!(f, "Parse Error: {}", e),
        }
    }
}

impl Error for FormatterError {}

impl From<io::Error> for FormatterError {
    fn from(err: io::Error) -> Self {
        FormatterError::IoError(err)
    }
}

impl From<serde_json::Error> for FormatterError {
    fn from(err: serde_json::Error) -> Self {
        FormatterError::ParseError(err.to_string())
    }
}

impl From<toml::de::Error> for FormatterError {
    fn from(err: toml::de::Error) -> Self {
        FormatterError::ParseError(err.to_string())
    }
}
