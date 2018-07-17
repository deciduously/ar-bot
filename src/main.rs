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
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate toml;

mod batch;
mod config;
mod errors {
    error_chain!{}
}

use clap::{App, Arg};
use config::*;
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

fn add(input_p: &str) -> Result<()> {
    let input = file_contents_from_str_path(input_p)?;
    println!("Input: {}", input); // TODO, obviously

    Ok(())
}

fn file_contents_from_str_path(s: &str) -> Result<String> {
    use std::{fs::File, path::Path};

    let f = File::open(Path::new(&s)).expect("Could not open input file");
    let mut bfr = BufReader::new(f);
    let mut input = String::new();
    let _ = bfr.read_to_string(&mut input);

    Ok(input)
}

fn run() -> Result<()> {
    let matches = App::new("ar-bot")
        .version("0.1.0")
        .author("deciduously <bendlovy@gmail.com>") // TODO read this from Cargo.toml?!
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
            Arg::with_name("config")
                .short("c")
                .long("config")
                .value_name("CONFIG_FILE")
                .takes_value(true)
                .help("Specify an alternate toml config file"),
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
            .expect("Could not read INPUT_FILE"))
            .chain_err(|| "Could not add input");
    }

    let config =
        init_config(matches.value_of("CONFIG_FILE")).chain_err(|| "Could not load configuration");
    println!("{:#?}", config);

    Ok(())
}
