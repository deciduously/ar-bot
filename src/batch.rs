// batch.rs handles the string parsing and batching logic for eliminating redundant line items
use errors::*;
use regex::Regex;
use std::{fmt, str::FromStr};

#[derive(Clone, Debug, PartialEq)]
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
            Other(s) => write!(f, "Non-builtin product {}", s),
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
        write!(f, "ID: {}, PRODUCT {}", self.id, self.product)
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
#[derive(Clone, Debug, PartialEq)]
pub struct BatchEntry {
    pub id: u32,
    pub products: Vec<Product>,
    //    pub times:
}

impl From<Entry> for BatchEntry {
    fn from(e: Entry) -> Self {
        BatchEntry {
            id: e.id,
            products: vec![e.product],
        }
    }
}

// The final batch
// there should only be one BatchEntry per ID - that's literally the whole point of this app
// Think about how to encode this constraint in the types
// Maybe a HashMap?
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

    pub fn add_entry(&mut self, e: Entry) -> Result<()> {
        // see if any BatchEntries share either an ID or both an ID and a product
        let mut duplicate_id = false;
        let mut duplicate_id_and_product = false;
        let mut existing_entry = None;

        let entries = self.entries.clone();

        for be in &entries {
            if be.id == e.id {
                duplicate_id = true;
                for p in &be.products {
                    if p == &e.product {
                        duplicate_id_and_product = true;
                        break;
                    }
                }
                existing_entry = Some(be.clone());
                break;
            }
        }

        println!("Inserting {}", e);

        // if it's not a duplicate, just insert it
        if !duplicate_id_and_product && !duplicate_id {
            self.entries.push(BatchEntry::from(e));
        } else if duplicate_id_and_product {
            // if it's a full duplicate, just insert the time
            // TODO times
        } else {
            // otherwise duplicate_id == true
            // add the product to the proper BatchEntry
            existing_entry.unwrap().products.push(e.product);
        }
        Ok(())
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

impl FromStr for Batch {
    type Err = Error;

    // TODO this is not correct
    // you'll likely want a direct BatchEntry::from_str()
    fn from_str(s: &str) -> Result<Self> {
        // each line is an entry
        let lines = s.split('\n');
        let mut entries = Vec::new();
        for line in lines {
            entries.push(BatchEntry::from(Entry::from_str(line)?));
        }
        Ok(Batch { entries })
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
