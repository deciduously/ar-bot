use errors::*;
use regex::Regex;
use std::{fmt, str::FromStr};

// represents a single email alert
#[derive(Debug, PartialEq)]
pub struct Entry {
    pub id: u32,
    pub product: String, // See if you can make this a Cow<'a, str>, maybe?  Lifetime issues abound
}

impl fmt::Display for Entry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "iMIS ID: {} - CHECK PRODUCT {}", self.id, self.product)
    }
}

impl FromStr for Entry {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        lazy_static! {
            static ref AD_RE: Regex = Regex::new(r"^The \w+ Invoice For iMIS ID (?P<id>\d+) For the Product (?P<product>.+) Has Changed You need to verify the Autodraft is now correct").unwrap();
        }

        if AD_RE.is_match(s) {
            let captures = AD_RE.captures(s).unwrap();
            return Ok(Entry {
                id: (&captures["id"])
                    .parse::<u32>()
                    .chain_err(|| "Could not read iMIS id")?,
                product: (&captures["product"]).into(),
            });
        } else {
            bail!("Couldn't match Regex")
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_entry_from_str() {
        use super::*;

        let input_str = "The Coolest Invoice For iMIS ID 12345 For the Product COOL_PROD Has Changed You need to verify the Autodraft is now correct";
        assert_eq!(
            Entry {
                id: 12345,
                product: "COOL_PROD".into()
            },
            Entry::from_str(input_str).unwrap()
        )
    }
}
