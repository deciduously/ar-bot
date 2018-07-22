// email.rs handles the input and output for the app

use email_format::rfc5322::Parsable;
use email_format::Email;
use errors::*;
use std::fmt;

#[derive(Debug)]
pub struct RawEmail {
    pub filename: String,
    pub contents: Email,
}

impl RawEmail {
    pub fn new(filename: &str, contents: &str) -> Result<Self> {
        debug!("PARSE: {}", contents);
        let (email, remainder) =
            Email::parse(contents.as_bytes()).chain_err(|| "Could not parse email")?;
        assert_eq!(remainder.len(), 0);
        Ok(RawEmail {
            filename: filename.into(),
            contents: email,
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
        let mut email = Email::new("iMIS@jccgb.org", "Sat, 21 Jul 2018 16:39:04 -0400")
            .chain_err(|| "Could not set email headers")?;
        email.set_body(s).chain_err(|| "Could not set email body")?;
        Ok(RawEmail {
            filename: format!("TEMPDATE.html"),
            contents: email,
        })
    }
}
