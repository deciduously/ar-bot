// cmd.rs holds the top-level commands, all returning errors::Result<_>
use brain::get_current_batch;
use clap::{App, Arg};
use config::init_config;
use errors::*;

use util::file_contents_from_str_path;

fn add(input_p: &str) -> Result<()> {
    let input = file_contents_from_str_path(input_p)?;
    println!("Input: {}", input); // TODO, obviously

    Ok(())
}

fn preview() -> Result<()> {
    let current_batch = get_current_batch()?;
    println!("{}", current_batch);
    Ok(())
}

// This is the entrypoint - essentially main()
pub fn run() -> Result<()> {
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
        init_config(matches.value_of("config")).chain_err(|| "Could not load configuration")?;
    println!("{}", config);

    if matches.is_present("preview") {
        preview()?;
    }

    Ok(())
}
