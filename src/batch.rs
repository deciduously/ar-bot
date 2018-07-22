// batch.rs handles the string parsing and batching logic for eliminating redundant line items
use brain::Brain;
use chrono::prelude::*;
use email::RawEmail;
use errors::*;
use regex::Regex;
use std::{collections::HashMap, fmt, str::FromStr};
#[cfg(test)]
use util::*;

// One instance of an email happening.
// Should it store the ID?  I'm only ever using this inside of a BatchEntry, with the id in the parent struct
#[derive(Clone, Debug, PartialEq)]
pub struct Alert {
    pub product: Product,
    pub times: Vec<DateTime<Utc>>,
}

impl Alert {
    fn new(product: Product, time: DateTime<Utc>) -> Self {
        Alert {
            product,
            times: vec![time],
        }
    }

    fn add_time(&mut self, t: DateTime<Utc>) {
        self.times.push(t);
    }
}

impl fmt::Display for Alert {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "_{} at {:?}_", self.product, self.times)
    }
}

// The final batch
// there should only be one BatchEntry per ID - that's literally the whole point of this app
// Think about how to encode this constraint in the types
// Maybe a HashMap?
#[derive(Debug, PartialEq)]
pub struct Batch {
    entries: Entries,
}

impl Batch {
    pub fn new() -> Self {
        Batch {
            entries: Entries::new(),
        }
    }

    pub fn add_entry(&mut self, e: Entry) -> Result<()> {
        let mut entry_class = EntryClass::default();
        let mut existing_id = None;

        // First, search for the id.  Only if we find it, search for a duplicate product on that id.

        for (id, be) in &self.entries {
            println!("Classifying {}", e);
            if *id == e.id {
                entry_class = EntryClass::NewProduct;
                for a in &be.alerts {
                    if e.product == a.product {
                        entry_class = EntryClass::Duplicate;
                        break;
                    }
                }
                existing_id = Some(*id);
                break;
            } else {
                entry_class = EntryClass::New;
            }
        }

        println!("Inserting {}", e);

        match entry_class {
            EntryClass::Duplicate => {
                // the only thing I push is the time, and I haven't done those yet
                // Multiple duplicate times are OK, I still wnat a note that I processed the email
                println!("This is a duplicate... just noting the new alert time");
                unimplemented!()
            }
            EntryClass::New => {
                println!("It's a brand new entry for this digest.");
                self.entries
                    .entry(existing_id.unwrap())
                    .or_insert(BatchEntry::from(e));
            }
            EntryClass::NewProduct => {
                // add the product to the proper BatchEntry
                // TODO Swap our clone back in place
                // For now, Im just pushing and pruning later

                println!("Same person, new product");
                let be = self.entries
                    .entry(existing_id.unwrap())
                    .or_insert(BatchEntry::default());
                be.alerts.push(Alert::new(e.product, e.time));
            }
            EntryClass::Unclassified => bail!("You shouldn't be hitting an Unclassified entry!"),
        }

        println!();
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
    pub fn test() -> Self {
        let mut batch = Batch::new();
        batch
            .add_entry(Entry::from_email(&RawEmail::from_str(TEST_COOL_STR).unwrap()).unwrap())
            .unwrap();
        batch
    }
    #[cfg(test)]
    pub fn test_second_email_str(s: &str) -> Self {
        let e = Entry::from_email(&RawEmail::from_str(TEST_COOL_STR).unwrap()).unwrap();
        let e_second = Entry::from_email(&RawEmail::from_str(s).unwrap()).unwrap();
        let mut entries = Entries::new();
        entries.entry(e.id).or_insert(BatchEntry::from(e));
        entries
            .entry(e_second.id)
            .or_insert(BatchEntry::from(e_second));
        Batch { entries }
    }
}

impl fmt::Display for Batch {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let _ = writeln!(f, "Current Batch:");
        if self.entries.is_empty() {
            write!(f, "No entries")
        } else {
            let entries_strs: Vec<String> =
                self.entries.iter().map(|(_, e)| format!("{}", e)).collect();
            let mut entries = String::new();
            for e in entries_strs {
                entries.push_str(&e);
            }
            write!(f, "{}", entries)
        }
    }
}

// Can store multiple entries
#[derive(Clone, Debug, Default, PartialEq)]
pub struct BatchEntry {
    pub id: u32,
    pub alerts: Vec<Alert>,
}

impl fmt::Display for BatchEntry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut alerts = String::new();
        for a in &self.alerts {
            alerts.push_str(&format!("{}", a));
        }
        writeln!(f, "{}: {}", self.id, alerts)
    }
}

impl From<Entry> for BatchEntry {
    fn from(e: Entry) -> Self {
        BatchEntry {
            id: e.id,
            alerts: vec![Alert::new(e.product, e.time)],
        }
    }
}

type Entries = HashMap<UserID, BatchEntry>;

#[derive(Debug, PartialEq)]
pub struct Entry {
    pub id: UserID,
    pub product: Product,
    pub time: DateTime<Utc>,
}

impl Entry {
    fn from_email(e: &RawEmail) -> Result<Self> {
        lazy_static! {
            static ref AD_RE: Regex = Regex::new(r"^The \w+ Invoice For iMIS ID (?P<id>\d+) For the Product (?P<product>\w+) Has Changed\r\nYou need to verify the Autodraft is now correct").unwrap();
        }

        let s = &format!(
            "{}",
            e.contents.get_body().chain_err(|| "No email body found")?
        );

        if AD_RE.is_match(s) {
            let captures = AD_RE.captures(s).unwrap();
            Ok(Entry {
                id: (&captures["id"])
                    .parse::<u32>()
                    .chain_err(|| "Could not read iMIS id")?,
                product: Product::from_str(&captures["product"])?,
                time: Utc.ymd(2000, 1, 1).and_hms(9, 10, 11), // Get this from the Email
            })
        } else {
            println!("{}", s);
            bail!("Couldn't match Regex")
        }
    }
}

impl fmt::Display for Entry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "ID {}, PRODUCT {}, TIME {}",
            self.id, self.product, self.time
        )
    }
}

#[derive(Debug)]
enum EntryClass {
    Duplicate,
    New,
    NewProduct,
    Unclassified,
}

impl Default for EntryClass {
    fn default() -> Self {
        EntryClass::Unclassified
    }
}

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

impl FromStr for Product {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        use self::Product::*;
        match s {
            "CG_TUITION" => Ok(CgBilling),
            "CG_TRANS" => Ok(CgTrans),
            "CAMP_KALE_TUIT" => Ok(CampKaleTuit),
            "CAMP_KALE_TRANS" => Ok(CampKaleTrans),
            "CAMP_KING_TUIT" => Ok(CampKingTuit),
            "CAMP_KING_TRANS" => Ok(CampKingTrans),
            _ => Ok(Other(s.into())),
        }
    }
}

type UserID = u32;

#[cfg(test)]
mod tests {
    use super::*;
    use email::RawEmail;

    #[test]
    fn test_entry_from_str() {
        assert_eq!(
            Entry::from_email(&RawEmail::from_str(TEST_COOL_STR).unwrap()).unwrap(),
            Entry {
                id: 12345,
                product: Product::Other(String::from("COOL_PROD")),
                time: Utc.ymd(2000, 1, 1).and_hms(9, 10, 11),
            },
        )
    }
    #[test]
    fn test_add_entry_to_empty_batch() {
        // Should create a new BatchEntry
        let mut batch = Batch::new();
        batch
            .add_entry(Entry::from_email(&RawEmail::from_str(TEST_COOL_STR).unwrap()).unwrap())
            .unwrap();
        let test_batch = Batch::test();
        assert_eq!(batch, test_batch)
    }
    #[test]
    fn test_add_entry_new_id() {
        // Should create a second BatchEntry
        let mut batch = Batch::new();
        batch
            .add_entry(Entry::from_email(&RawEmail::from_str(TEST_COOL_STR).unwrap()).unwrap())
            .unwrap();
        batch
            .add_entry(Entry::from_email(&RawEmail::from_str(TEST_DIF_BOTH).unwrap()).unwrap())
            .unwrap();
        let test_batch = Batch::test_second_email_str(TEST_DIF_BOTH);
        assert_eq!(batch, test_batch)
    }
    #[test]
    fn test_add_entry_duplicate_id() {
        // Should add product to existing BatchEntry
        let mut batch = Batch::new();
        batch
            .add_entry(Entry::from_email(&RawEmail::from_str(TEST_COOL_STR).unwrap()).unwrap())
            .unwrap();
        batch
            .add_entry(Entry::from_email(&RawEmail::from_str(TEST_DIF_PROD).unwrap()).unwrap())
            .unwrap();
        let test_batch = Batch::test_second_email_str(TEST_DIF_PROD);
        assert_eq!(batch, test_batch)
    }
    #[test]
    fn test_add_entry_duplicate_product() {
        // Make a BatchEntry for a the new ID, it doesnt matter if two products are the same
        let mut batch = Batch::new();
        batch
            .add_entry(Entry::from_email(&RawEmail::from_str(TEST_COOL_STR).unwrap()).unwrap())
            .unwrap();
        batch
            .add_entry(Entry::from_email(&RawEmail::from_str(TEST_DIF_ID).unwrap()).unwrap())
            .unwrap();
        let test_batch = Batch::test_second_email_str(TEST_DIF_ID);
        assert_eq!(batch, test_batch)
    }
    #[test]
    fn test_add_entry_duplicate_id_and_product() {
        // Should just add the time
        let mut batch = Batch::new();
        batch
            .add_entry(Entry::from_email(&RawEmail::from_str(TEST_COOL_STR).unwrap()).unwrap())
            .unwrap();
        batch
            .add_entry(Entry::from_email(&RawEmail::from_str(TEST_COOL_STR).unwrap()).unwrap())
            .unwrap();
        let test_batch = Batch::test_second_email_str(TEST_COOL_STR);
        assert_eq!(batch, test_batch)
    }
}
