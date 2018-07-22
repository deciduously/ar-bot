// util.rs holds, well, utility functions
use errors::*;
use std::io::{prelude::*, BufReader};

#[cfg(test)]
pub static TEST_COOL_STR: &'static str = "The Cool Invoice For iMIS ID 12345 For the Product COOL_PROD Has Changed\r\nYou need to verify the Autodraft is now correct";
#[cfg(test)]
pub static TEST_DIF_ID: &'static str = "The Cool Invoice For iMIS ID 12346 For the Product COOL_PROD Has Changed\r\nYou need to verify the Autodraft is now correct";
#[cfg(test)]
pub static TEST_DIF_PROD: &'static str = "The Cool Invoice For iMIS ID 12345 For the Product COOL_PROOD Has Changed\r\nYou need to verify the Autodraft is now correct";
#[cfg(test)]
pub static TEST_DIF_BOTH: &'static str = "The Cool Invoice For iMIS ID 22345 For the Product COL_PROD Has Changed\r\nYou need to verify the Autodraft is now correct";

pub fn file_contents_from_str_path(s: &str) -> Result<String> {
    use std::{fs::File, path::Path};

    let f = File::open(Path::new(&s))
        .chain_err(|| "file_contents_from_str_path could not open input file")?;
    let mut bfr = BufReader::new(f);
    let mut input = String::new();
    let _ = bfr.read_to_string(&mut input);

    Ok(input)
}
