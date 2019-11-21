// email.rs handles the input and output for the app

// I dont think you need email_format - rip it out.
// we're just passing around strings.
use crate::errors::*;
use log::*;
use std::fmt;
//use util::DATE_OUT_FMT;


// Instead of just the contents, this should have the proper fields
// filename, date, id, product
// except that's exactly what a batch::Entry is
#[derive(Debug)]
pub struct Email {
    pub filename: String,
    pub contents: String
}

impl Email {
    pub fn new(filename: &str, contents: &str) -> Result<Self> {
        debug!("EMAIL FOUND: {}", contents);
        Ok(Email {
            filename: filename.into(),
            contents: contents.into(),
        })
    }
}

impl fmt::Display for Email {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.contents)
    }
}

// This is only used for testing
#[cfg(test)]
impl std::str::FromStr for Email {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut email_contents = String::from("From: iMIS <iMIS@jccgb.org>\r\nSent: Saturday, July 21, 2018 4:39 PM\r\nTo: Some People; Whose Names; Are Omitted\r\nSubject: Invoice Charge Change for Super Nifty Autodraft\r\n\r\n");
        email_contents.push_str(s);
        println!("RAWFROMSTR: {}", email_contents);
        Ok(Email {
            filename: format!("Saturday, July 21, 2018 4:39 PM.html"),
            contents: email_contents,
        })
    }
}
