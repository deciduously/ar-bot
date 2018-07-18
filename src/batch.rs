// batch.rs handles the string parsing and batching logic for eliminating redundant line items
use errors::*;
use regex::Regex;
use std::{fmt, str::FromStr};

#[derive(Debug, PartialEq)]
pub enum Product {
    CgBilling,
    CgTrans,
    CampKaleTuit,
    CampKaleTrans,
    CampKingTuit,
    CampKingTrans,
    Other(String),
}

impl FromStr for Product {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        use self::Product::*;
        match s {
            "CG_BILLING" => Ok(CgBilling),
            "CG_TRANS" => Ok(CgTrans),
            "CAMP_KALE_TUIT" => Ok(CampKaleTuit),
            "CAMP_KALE_TRANS" => Ok(CampKaleTrans),
            "CAMP_KING_TUIT" => Ok(CampKingTuit),
            "CAMP_KING_TRANS" => Ok(CampKingTrans),
            _ => Ok(Other(s.into())),
        }
    }
}

impl fmt::Display for Product {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Product::*;

        match self {
            CgBilling => write!(f, "Grossman Tuition"),
            CgTrans => write!(f, "Grossman Tranportation"),
            CampKaleTuit => write!(f, "Kaleidoscope Tuition"),
            CampKaleTrans => write!(f, "Kaleidoscope Transportation"),
            CampKingTuit => write!(f, "Kingswood Tuition"),
            CampKingTrans => write!(f, "Kingswood Transportation"),
            Other(s) => write!(f, "Non-builtin: {}", s),
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
        write!(f, "ID: {}, PRODUCT {:?}", self.id, self.product)
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
            Ok(Entry {
                id: (&captures["id"])
                    .parse::<u32>()
                    .chain_err(|| "Could not read iMIS id")?,
                product: Product::from_str(&captures["product"])?,
            })
        } else {
            bail!("Couldn't match Regex")
        }
    }
}

// Can store multiple entries
//
#[derive(Debug, PartialEq)]
pub struct BatchEntry {
    pub id: u32,
    pub products: Vec<Product>,
    //    pub times:
}

// The final batch
#[derive(Debug, PartialEq)]
pub struct Batch {
    entries: Vec<BatchEntry>,
}

impl Batch {
    pub fn new() -> Self {
        Batch {
            entries: Vec::new(),
        }
    }
}

impl fmt::Display for Batch {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let _ = writeln!(f, "Current Batch:");
        if self.entries.is_empty() {
            write!(f, "No entries")
        } else {
            let entries_strs: Vec<String> =
                self.entries.iter().map(|e| format!("{:#?}", e)).collect();
            let mut entries = String::new();
            for e in entries_strs {
                entries.push_str(&e);
            }
            write!(f, "{}", entries)
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_entry_from_str() {
        use super::*;

        let input_str = "The Cool Invoice For iMIS ID 12345 For the Product COOL_PROD Has Changed You need to verify the Autodraft is now correct";
        assert_eq!(
            Entry {
                id: 12345,
                product: Product::Other(String::from("COOL_PROD")),
            },
            Entry::from_str(input_str).unwrap()
        )
    }
}
