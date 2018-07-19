// main.rs is the entry point of the executable
#![recursion_limit = "1024"]

extern crate chrono;
extern crate clap;
#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate lazy_static;
extern crate lettre;
extern crate lettre_email;
#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;
#[cfg(test)]
extern crate rand;
extern crate regex;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate toml;

mod batch;
mod brain;
mod cmd;
mod config;
mod email;
mod errors {
    error_chain!{}
}
mod util;

use cmd::run;

fn main() {
    // Immediately call into a properly error-chained fn
    if let Err(ref e) = run() {
        eprintln!("error: {}", e);

        for e in e.iter().skip(1) {
            eprintln!("caused by: {}", e);
        }

        if let Some(backtrace) = e.backtrace() {
            eprintln!("backtrace: {:?}", backtrace);
        }

        ::std::process::exit(1);
    }
}
