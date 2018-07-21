// email.rs handles the input and output for the app

use email_format::rfc5322::Parsable;
use email_format::Email;
use errors::*;
use std::fmt;

#[derive(Debug, PartialEq)]
pub struct RawEmail {
    pub filename: String,
    pub contents: String,
}

impl RawEmail {
    pub fn new(filename: &str, contents: &str) -> Result<Self> {
        Ok(RawEmail {
            filename: filename.into(),
            contents: contents.into(),
        })
    }
}

impl fmt::Display for RawEmail {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.contents)
    }
}

// This is only used for testing
#[cfg(test)]
impl ::std::str::FromStr for RawEmail {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        Ok(RawEmail {
            filename: format!("TEMPDATE.html"),
            contents: s.into(),
        })
    }
}
