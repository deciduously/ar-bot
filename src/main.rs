#![recursion_limit = "1024"]

extern crate clap;
#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate lazy_static;
#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;
extern crate regex;

mod batch;
mod errors {
    error_chain!{}
}

use clap::{App, Arg};
use errors::*;
use std::io::{prelude::*, BufReader};

fn main() {
    if let Err(ref e) = run() {
        println!("error: {}", e);

        for e in e.iter().skip(1) {
            println!("caused by: {}", e);
        }

        if let Some(backtrace) = e.backtrace() {
            println!("backtrace: {:?}", backtrace);
        }

        ::std::process::exit(1);
    }
}

fn add(input_path: &str) -> Result<()> {
    use std::{fs::File, path::Path};
    let f = File::open(Path::new(&input_path)).expect("Could not open input file");
    let mut bfr = BufReader::new(f);
    let mut input = String::new();
    let _ = bfr.read_to_string(&mut input);
    println!("Input: {}", input);
    Ok(())
}

fn run() -> Result<()> {
    let matches = App::new("ar-bot")
        .version("0.1.0")
        .author("deciduously <bendlovy@gmail.com>")
        .about("Batching of auto email alerts")
        .arg(
            Arg::with_name("add")
                .short("a")
                .long("add")
                .value_name("INPUT_FILE")
                .help("Add a new file to the register")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("preview")
                .short("p")
                .long("preview")
                .takes_value(false)
                .help("Displays the current contents of the batch"),
        )
        .arg(
            Arg::with_name("report")
                .short("r")
                .long("report")
                .takes_value(false)
                .help("Daily report comparing inputs to outputs for the day"),
        )
        .get_matches();

    if matches.is_present("add") {
        let _ = add(matches
            .value_of("INPUT_FILE")
            .expect("Must provide input file"))
            .chain_err(|| "Could not add input");
    }

    Ok(())
}
