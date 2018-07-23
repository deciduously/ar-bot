// main.rs is the entry point of the executable
#![recursion_limit = "1024"]

#[macro_use]
extern crate askama;
extern crate chrono;
extern crate clap;
extern crate email_format;
#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
// extern crate lettre;
// extern crate lettre_email;
#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;
extern crate pretty_env_logger;
#[cfg(test)]
extern crate rand;
extern crate regex;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate toml;
extern crate uuid;

mod batch;
mod brain;
mod cmd;
mod config;
mod email;
mod errors {
    error_chain!{}
}
mod page;
mod util;

use cmd::run;

fn main() {
    // Immediately call into a properly error-chained fn
    if let Err(ref e) = run() {
        error!("error: {}", e);

        for e in e.iter().skip(1) {
            debug!("caused by: {}", e);
        }

        if let Some(backtrace) = e.backtrace() {
            trace!("backtrace: {:?}", backtrace);
        }

        ::std::process::exit(1);
    }
}
