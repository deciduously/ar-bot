// cmd.rs holds the top-level commands, all returning errors::Result<_>
use batch::Batch;
use brain::Context;
use clap::{App, Arg};
use config::init_config;
//use email::email;
use errors::*;

static VERSION: &'static str = "0.1.0";

// Outputs the batch to the console
// This just reads the emails in the folder and displays what the digest would look like
// if we ran that command now, but makes no changes.
fn preview(ctx: &mut Context) -> Result<()> {
    println!("{}\n", Batch::from_brain(&ctx.brain)?);
    Ok(())
}

// Unimplemented!  This is a placeholder
// TODO this will actually write out the digest, and copy everything written
// It should ideally ask user to confirm
// For now, this doesn't need to be automatic.
// For MVP demo, just have an outlook folder that you can see.
// When it gets to a certain number, run the program
// save a zipped folder of the emails `emails.zip`
// save the digest to `DATETIME.digest.html`
// REPORT AND DIGEST NEED TO BE DIFFERENT COMMANDS
fn report(_ctx: &Context) -> Result<()> {
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
        // This one will be a subcommand with subcommands
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .value_name("CONFIG_FILE")
                .takes_value(true)
                .help("Specify an alternate toml config file"),
        )
        //.arg(
        //    Arg::with_name("email")
        //        .short("e")
        //        .long("email")
        //        .takes_value(false)
        //        .help("Placeholder command for developing email functionality"),
        //)
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

    // Initialize configuration and read in Brain
    let config = init_config(matches.value_of("config"))
        .chain_err(|| "Could not load configuration file")
        .chain_err(|| "Could not make heads or tails of that abomination of a config file")?;
    println!("{}\n", config);

    // Grab a Context with a Brain
    // this takes ownership of Config - all further access is via this ctx
    // Because Rust is great, everything will clean itslef up nicely when ctx goes out of scope
    let mut ctx = Context::initialize(config).chain_err(|| "Could not initialze config")?;

    // Call relative functions if their respective flags are present.
    // TODO smart preview with add - how SHOULD it be?
    // For now, I'm calling preview last so that we always dislay the end result of the batch

    //if matches.is_present("email") {
    //    email()?;
    //}

    if matches.is_present("report") {
        report(&ctx)?;
    }

    if matches.is_present("preview") {
        preview(&mut ctx)?;
    }

    println!("Goodbye!");

    Ok(())
}
