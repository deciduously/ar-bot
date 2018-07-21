// batch.rs handles the string parsing and batching logic for eliminating redundant line items
use brain::{Brain, Email};
use chrono::prelude::*;
use errors::*;
use regex::Regex;
use std::{fmt, str::FromStr};
#[cfg(test)]
use util::TEST_COOL_STR;

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
    pub time: DateTime<Utc>,
}

impl Entry {
    // For testing purposes
    #[cfg(test)]
    pub fn test() -> Self {
        Entry {
            id: 12345,
            product: Product::from_str("COOL_PROD").unwrap(),
            time: Utc.ymd(2000, 1, 1).and_hms(9, 10, 11),
        }
    }
}

impl Entry {
    // TODO this will eventually be getting a whole email
    // including headers, time, etc
    // instead of Utc::now(), store whatever time the email was received
    // For now, this is close enough
    fn from_email(e: &Email) -> Result<Self> {
        lazy_static! {
            static ref AD_RE: Regex = Regex::new(r"^The \w+ Invoice For iMIS ID (?P<id>\d+) For the Product (?P<product>.+) Has Changed You need to verify the Autodraft is now correct").unwrap();
        }

        let s = &e.contents;

        if AD_RE.is_match(s) {
            let captures = AD_RE.captures(s).unwrap();
            Ok(Entry {
                id: (&captures["id"])
                    .parse::<u32>()
                    .chain_err(|| "Could not read iMIS id")?,
                product: Product::from_str(&captures["product"])?,
                time: Utc.ymd(2000, 1, 1).and_hms(9, 10, 11),
            })
        } else {
            bail!("Couldn't match Regex")
        }
    }
}

impl fmt::Display for Entry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ID: {}, PRODUCT {}", self.id, self.product)
    }
}
// Can store multiple entries
#[derive(Clone, Debug, PartialEq)]
pub struct BatchEntry {
    pub id: u32,
    pub products: Vec<Product>,
    pub times: Vec<DateTime<Utc>>,
}

impl fmt::Display for BatchEntry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {:?} at {:?}", self.id, self.products, self.times)
    }
}

impl From<Entry> for BatchEntry {
    fn from(e: Entry) -> Self {
        BatchEntry {
            id: e.id,
            products: vec![e.product],
            times: vec![e.time],
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
            // the only thing I push is the time, and I haven't done those yet
            unimplemented!()
        } else {
            // otherwise duplicate_id == true
            // add the product to the proper BatchEntry
            existing_entry.unwrap().products.push(e.product);
        }
        Ok(())
    }

    pub fn from_brain(brain: &Brain) -> Result<Self> {
        // call add_entry on each email in the brain
        let mut ret = Batch::new();
        for email in &brain.emails {
            ret.add_entry(Entry::from_email(email)?)?;
        }
        Ok(ret)
    }

    #[cfg(test)]
    // Test batch with one entry inserted
    pub fn test() -> Self {
        let mut batch = Batch::new();
        batch
            .add_entry(Entry::from_email(&Email::from_str(TEST_COOL_STR).unwrap()).unwrap())
            .unwrap();
        batch
    }
}

impl fmt::Display for Batch {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let _ = writeln!(f, "Current Batch:");
        if self.entries.is_empty() {
            write!(f, "No entries")
        } else {
            let entries_strs: Vec<String> = self.entries.iter().map(|e| format!("{}", e)).collect();
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
            entries.push(BatchEntry::from(Entry::from_email(&Email::from_str(
                line,
            )?)?));
        }
        Ok(Batch { entries })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_entry_from_str() {
        assert_eq!(
            Entry::from_email(&Email::from_str(TEST_COOL_STR).unwrap()).unwrap(),
            Entry {
                id: 12345,
                product: Product::Other(String::from("COOL_PROD")),
                time: Utc.ymd(2000, 1, 1).and_hms(9, 10, 11),
            },
        )
    }
    #[test]
    fn test_add_entry_to_empty_batch() {
        let mut batch = Batch::new();
        batch.add_entry(Entry::test()).unwrap();
        let test_batch = Batch::test();
        assert_eq!(batch, test_batch)
    }
    //#[test]
    //fn test_add_entry_duplicate_id() {
    //    assert_eq!("Write", "Me")
    //}
    //#[test]
    //fn test_add_entry_duplicate_id_and_product() {
    //    assert_eq!("Write", "Me"
    //}
}
