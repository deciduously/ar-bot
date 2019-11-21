// main.rs is the entry point of the executable
#![recursion_limit = "1024"]

mod batch;
mod brain;
mod cmd;
mod config;
mod email;
mod errors {
    use error_chain::error_chain;
    error_chain!{}
}
mod page;
mod util;

use cmd::run;
use log::*;

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
