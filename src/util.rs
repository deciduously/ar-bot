// util.rs holds stuff I use in a few differnt places, basically.  Sue me.
use errors::*;
use std::io::{prelude::*, BufReader};

pub fn file_contents_from_str_path(s: &str) -> Result<String> {
    use std::{fs::File, path::Path};

    let f = File::open(Path::new(&s)).expect("Could not open input file");
    let mut bfr = BufReader::new(f);
    let mut input = String::new();
    let _ = bfr.read_to_string(&mut input);

    Ok(input)
}
