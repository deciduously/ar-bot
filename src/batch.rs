// batch.rs handles the string parsing and batching logic for eliminating redundant line items
use brain::Brain;
//use chrono::prelude::*;
use email::RawEmail;
use errors::*;
use log::Level;
use regex::Regex;
use std::{collections::HashMap, fmt, str::FromStr};
#[cfg(test)]
use util::*;

type Alerts = HashMap<Product, Vec<String>>;

// The final batch
// there should only be one BatchEntry per ID - that's literally the whole point of this app
// Think about how to encode this constraint in the types
// Maybe a HashMap?
#[derive(Debug, PartialEq)]
pub struct Batch {
    pub entries: Entries,
}

impl Batch {
    pub fn new() -> Self {
        Batch {
            entries: Entries::new(),
        }
    }

    pub fn add_entry(&mut self, e: Entry) -> Result<()> {
        let entry_class = self.classify(&e);

        // First, search for the id.  Only if we find it, search for a duplicate product on that id.

        let mut info_str = String::new();
        if log_enabled!(Level::Info) {
            info_str.push_str(&format!("INSERT: {} <", e));
        }

        match entry_class {
            EntryClass::Duplicate((id, product)) => {
                // the only thing I push is the time, and I haven't done those yet
                // Multiple duplicate times are OK, I still wnat a note that I processed the email
                if log_enabled!(Level::Info) {
                    info_str.push_str("This is a duplicate... just noting the new alert time");
                }

                for (uid, batch_entry) in &mut self.entries {
                    if id == *uid {
                        for (key, times) in &mut batch_entry.alerts {
                            if *key == product {
                                times.push(e.time.clone());
                            }
                        }
                    }
                }
            }
            EntryClass::New => {
                if log_enabled!(Level::Info) {
                    info_str.push_str("It's a brand new entry for this digest.");
                }
                self.entries
                    .entry(e.id)
                    .or_insert_with(|| BatchEntry::from(e));
            }
            EntryClass::NewProduct(id) => {
                // add the product to the proper BatchEntry
                // TODO Swap our clone back in place
                // For now, Im just pushing and pruning later

                if log_enabled!(Level::Info) {
                    info_str.push_str("Same person, new product");
                }

                for (uid, batch_entry) in &mut self.entries {
                    if id == *uid {
                        let times = vec![e.time.clone()];
                        batch_entry.alerts.entry(e.product.clone()).or_insert(times);
                    }
                }
            }
        }
        if log_enabled!(Level::Info) {
            info_str.push_str(">");
            info!("{}", info_str);
        }
        Ok(())
    }

    // Classifies an entry
    fn classify(&self, e: &Entry) -> EntryClass {
        let mut entry_class = EntryClass::default();
        for (id, be) in &self.entries {
            if *id == e.id {
                entry_class = EntryClass::NewProduct(*id);
                for p in be.alerts.keys() {
                    if e.product == *p {
                        entry_class = EntryClass::Duplicate((*id, e.product.clone()));
                        break;
                    }
                }
                break;
            }
        }
        debug!("CLASSIFY: {} - {:?}", e, entry_class);
        entry_class
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
    pub alerts: Alerts,
}

impl BatchEntry {
    // Just returns the alerts as a vector of (Product, Vec<DateTime<Local>>)
    // is this necessary?
    pub fn alerts_vec(&self) -> Vec<(Product, Vec<String>)> {
        let mut ret = Vec::new();
        for (k, v) in &self.alerts {
            ret.push((k.clone(), v.clone()));
        }
        ret
    }
}

impl fmt::Display for BatchEntry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut alerts = String::new();
        for (k, ts) in &self.alerts {
            let times: Vec<String> = ts.iter().map(|t| format!("{}, ", t)).collect();
            let mut time_str = String::new();
            for t in times {
                time_str.push_str(&t);
            }
            // TODO squash times.
            alerts.push_str(&format!("{} at {}", k, time_str));
        }
        writeln!(f, "{}: {}", self.id, alerts)
    }
}

impl From<Entry> for BatchEntry {
    fn from(e: Entry) -> Self {
        let mut alerts = Alerts::new();
        let times = vec![e.time];
        alerts.entry(e.product).or_insert(times);
        BatchEntry { id: e.id, alerts }
    }
}

type Entries = HashMap<UserID, BatchEntry>;

#[derive(Debug, PartialEq)]
pub struct Entry {
    pub id: UserID,
    pub product: Product,
    pub time: String,
}

impl Entry {
    fn from_email(e: &RawEmail) -> Result<Self> {
        lazy_static! {
            static ref AD_RE: Regex = Regex::new(r"^The \w+ Invoice For iMIS ID (?P<id>\d+) For the Product (?P<product>\w+) Has Changed\r\nYou need to verify the Autodraft is now correct").unwrap();
            static ref DATE_RE: Regex = Regex::new(r"^Date:( )*(?P<date>.+)\r\n").unwrap();
        }

        let s = &format!(
            "{}",
            e.contents.get_body().chain_err(|| "No email body found")?
        );

        // TODO am I allowed to impl From<email_format::rfc5322::OrigDate> for chrono::DateTime without going through &str?
        let datetime_str_raw = format!("{}", e.contents.get_date());
        // get_date() returns "Date: `rfc2822_str`" so I skip "Date: " and the trailing \r\n
        // This is just for the demo
        // TODO write a proper regex
        let dt_captures = DATE_RE.captures(&datetime_str_raw).unwrap();
        let datetime_str = &dt_captures["date"];

        debug!("DTSTR: {}", datetime_str);
        // DateTime::parse_from_rfc2822 is available, but I'm not positive that's what this is
        //let dt: DateTime<FixedOffset> = DateTime::parse_from_rfc2822(&datetime_str)
        //    .chain_err(|| format!("Date in email {} was not rfc2822 formatted", e.filename))?;
        if AD_RE.is_match(s) {
            debug!("MATCH: {}", s);
            let ad_captures = AD_RE.captures(s).unwrap();
            Ok(Entry {
                id: (&ad_captures["id"])
                    .parse::<u32>()
                    .chain_err(|| "Could not read iMIS id")?,
                product: Product::from_str(&ad_captures["product"])?,
                time: datetime_str.to_string(),
            })
        } else {
            debug!("{}", s);
            bail!("Couldn't match Regex")
        }
    }
}

impl fmt::Display for Entry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "ID {}, PRODUCT {}, TIME {}",
            self.id, self.product, self.time,
        )
    }
}

#[derive(Debug, PartialEq)]
enum EntryClass {
    Duplicate((UserID, Product)),
    New,
    NewProduct(UserID),
}

impl Default for EntryClass {
    fn default() -> Self {
        EntryClass::New
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
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
                time: "Sat, 21 Jul 2018 16:39:04 -0400".to_string(),
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
        let test_time = "Sat, 21 Jul 2018 16:39:04 -0400".to_string();
        // Should add product to existing BatchEntry
        let mut batch = Batch::new();
        batch
            .add_entry(Entry::from_email(&RawEmail::from_str(TEST_COOL_STR).unwrap()).unwrap())
            .unwrap();
        batch
            .add_entry(Entry::from_email(&RawEmail::from_str(TEST_DIF_PROD).unwrap()).unwrap())
            .unwrap();
        let mut test_alerts = HashMap::new();
        test_alerts
            .entry(Product::Other("COOL_PROOD".into()))
            .or_insert(vec![test_time.clone()]);
        test_alerts
            .entry(Product::Other("COOL_PROD".into()))
            .or_insert(vec![test_time.clone()]);
        let test_batch_entry = BatchEntry {
            id: 12345,
            alerts: test_alerts,
        };
        let mut test_entries = HashMap::new();
        test_entries.entry(12345).or_insert(test_batch_entry);
        let test_batch = Batch {
            entries: test_entries,
        };

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
        let test_batch = Batch::test_second_email_str(TEST_DIF_ID); // Bath::test_second_email_str isn't smart like that
        assert_eq!(batch, test_batch)
    }
    #[test]
    fn test_add_entry_duplicate_id_and_product() {
        // Should just add the time
        let test_time = "Sat, 21 Jul 2018 16:39:04 -0400".to_string();
        let mut batch = Batch::new();
        batch
            .add_entry(Entry::from_email(&RawEmail::from_str(TEST_COOL_STR).unwrap()).unwrap())
            .unwrap();
        batch
            .add_entry(Entry::from_email(&RawEmail::from_str(TEST_COOL_STR).unwrap()).unwrap())
            .unwrap();
        //let test_batch = Batch::test_second_email_str(TEST_COOL_STR);
        let mut test_alerts = HashMap::new();
        let test_times = vec![test_time.clone(), test_time.clone()];
        test_alerts
            .entry(Product::Other("COOL_PROD".into()))
            .or_insert(test_times);
        let test_batch_entry = BatchEntry {
            id: 12345,
            alerts: test_alerts,
        };
        let mut test_entries = HashMap::new();
        test_entries.entry(12345).or_insert(test_batch_entry);
        let test_batch = Batch {
            entries: test_entries,
        };

        assert_eq!(batch, test_batch)
    }
    #[test]
    fn test_classify_new_entry() {
        let test_batch = Batch::test();
        let test_entry = Entry::from_email(&RawEmail::from_str(TEST_DIF_BOTH).unwrap()).unwrap();
        assert_eq!(test_batch.classify(&test_entry), EntryClass::New)
    }
    #[test]
    fn test_classify_duplicate_id_and_product() {
        let test_batch = Batch::test();
        let test_entry = Entry::from_email(&RawEmail::from_str(TEST_COOL_STR).unwrap()).unwrap();
        assert_eq!(
            test_batch.classify(&test_entry),
            EntryClass::Duplicate((12345, Product::from_str("COOL_PROD").unwrap()))
        )
    }
    #[test]
    fn test_classify_duplicate_id() {
        let test_batch = Batch::test();
        let test_entry = Entry::from_email(&RawEmail::from_str(TEST_DIF_PROD).unwrap()).unwrap();
        assert_eq!(
            test_batch.classify(&test_entry),
            EntryClass::NewProduct(12345)
        )
    }
    #[test]
    fn test_classify_duplicate_prod() {
        let test_batch = Batch::test();
        let test_entry = Entry::from_email(&RawEmail::from_str(TEST_DIF_ID).unwrap()).unwrap();
        assert_eq!(test_batch.classify(&test_entry), EntryClass::New)
    }
}
