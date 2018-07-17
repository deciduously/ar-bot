use errors::*;
use std::{fmt, str::FromStr};

// represents a single email alert
#[derive(Debug, PartialEq)]
pub struct Entry {
    pub id: u32,
    pub product: String, // See if you can make this a Cow<'a, str> - you ran into a lifetime issue in from_str
}

impl fmt::Display for Entry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ID: {} - CHECK PRODUCT {}", self.id, self.product)
    }
}

impl FromStr for Entry {
    type Err = Error;
    
    fn from_str(s: &str) -> Result<Self> {
        Ok(Entry { id: 0, product: s.into()})
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_entry_from_str() {
        use super::*;
        
        let input_str = "The Coolest Invoice for iMIS ID 12345 For Product COOL_PROD Has Changed,\nYou Need To Ensure The Autodraft Is Correct";
        assert_eq!(Entry { id: 12345, product: "COOL_PROD".into()}, Entry::from_str(input_str).unwrap())
    }
}
