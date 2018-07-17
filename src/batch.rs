use errors::*;
use regex::Regex;
use std::{fmt, str::FromStr};

#[derive(Debug, PartialEq)]
pub enum Product {
    CG_BILLING,
    CG_TRANS,
    CAMP_KALE_TUIT,
    CAMP_KALE_TRANS,
    CAMP_KING_TUIT,
    CAMP_KING_TRANS,
    Other,
}

impl FromStr for Product {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        use self::Product::*;
        match s {
            "CG_BILLING" => Ok(CG_BILLING),
            "CG_TRANS" => Ok(CG_TRANS),
            "CAMP_KALE_TUIT" => Ok(CAMP_KALE_TUIT),
            "CAMP_KALE_TRANS" => Ok(CAMP_KALE_TRANS),
            "CAMP_KING_TUIT" => Ok(CAMP_KALE_TUIT),
            "CAMP_KING_TRANS" => Ok(CAMP_KING_TRANS),
            _ => Ok(Other),
        }
    }
}

// represents a single email alert
#[derive(Debug, PartialEq)]
pub struct Entry {
    pub id: u32,
    pub product: Product,
    // pub time: Something,
}

impl fmt::Display for Entry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "iMIS ID: {} - CHECK PRODUCT {:?}", self.id, self.product)
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
                product: Product::from_str(&captures["product"])?,
            });
        } else {
            bail!("Couldn't match Regex")
        }
    }
}

// Can store multiple entries
//
#[derive(Debug)]
pub struct BatchedEntry {
    pub id: u32,
    pub products: Vec<Product>,
//    pub times:
}

// The final batch
pub struct Batch {
    entries: Vec<BatchedEntry>,
}

impl Batch {
    pub fn new() -> Self {
        Batch { entries: Vec::new() }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_entry_from_str() {
        use super::*;

        let input_str = "The Grossman Invoice For iMIS ID 12345 For the Product CG_BILLING Has Changed You need to verify the Autodraft is now correct";
        assert_eq!(
            Entry {
                id: 12345,
                product: Product::CG_BILLING,
            },
            Entry::from_str(input_str).unwrap()
        )
    }
}
