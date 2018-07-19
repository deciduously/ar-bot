// cmd.rs holds the top-level commands, all returning errors::Result<_>
use batch::Entry;
use brain::Brain;
use clap::{App, Arg};
use config::{init_config, Config};
use errors::*;
use std::str::FromStr;

use util::file_contents_from_str_path;

static VERSION: &'static str = "0.1.0";

// Takes the given file path relative to crate root and adds its contents to the batch
fn add(c: &Config, input_p: &str) -> Result<()> {
    let mut brain = Brain::get_all(c)?;
    let input = file_contents_from_str_path(input_p)?;
    let entry = Entry::from_str(&input)?;
    brain.add_entry(entry, c)?;
    Ok(())
}

// Outputs the batch to the console
fn preview(c: &Config) -> Result<()> {
    let current_brain = Brain::get_all(c)?;
    println!("{}\n", current_brain.batch);
    Ok(())
}

// Unimplemented!  This is a placeholder
fn report(_c: &Config) -> Result<()> {
    // TODO
    println!("AR-Bot Daily Report for <DATE>\nGenerated at <TIME>\n\nNothing to report.\n");

    Ok(())
}

// This is the entrypoint - essentially main()
pub fn run() -> Result<()> {
    // clap config
    let matches = App::new("ar-bot")
        .version(VERSION)
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
        // This one will be a subcommand with subcommands
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
        // Arg cleanup
        // Arg search_hx - maybe use ripgrep!
        // Arg send
        .get_matches();

    println!("AR-Bot v.{}\npass '-h' or '--help' for usage\n", VERSION);

    let config = init_config(matches.value_of("config"))
        .chain_err(|| "Could not load configuration file")
        .chain_err(|| "Could not make heads or tails of that abomination of a config file")?;
    println!("{}\n", config);

    // TODO, instead of just a Config, pass around a Context with that and the Brain

    // Call relative functions if their respective flags are present.
    // TODO smart preview with add - how SHOULD it be?
    // For now, I'm calling preview last so that we always dislay the end result of the batch

    if matches.is_present("add") {
        let _ = add(
            &config,
            matches.value_of("add").expect("Could not read INPUT_FILE"),
        ).chain_err(|| "Could not add input");
    }

    if matches.is_present("report") {
        report(&config)?;
    }

    if matches.is_present("preview") {
        preview(&config)?;
    }

    println!("Goodbye!");

    Ok(())
}
