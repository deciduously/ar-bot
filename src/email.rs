// email.rs handles the input and output for the app
// BEN disconnnect this for now and do it last.

use errors::*;
use lettre::{EmailTransport, SmtpTransport};
use lettre_email::EmailBuilder;
use std::path::Path;

pub fn email() -> Result<()> {
    let email = EmailBuilder::new()
        // can either use tuple or just addr
        .to(("blovy@jccgb.org", "Ben Lovy"))
        .from("ar-bot@dinosaur.com")
        .subject("Rust made me")
        .text("And it feels sooo good")
        .build().chain_err(|| "Failed to build hardcoded email")?;

    // open local connection on port 25
    let mut mailer = SmtpTransport::builder_unencrypted_localhost()
        .chain_err(|| "Failed to build SmtpTransport")?
        .build();
    let result = mailer.send(&email);
    if result.is_ok() {
        println!("Sent email");
    } else {
        println!("Could not send email: {:?}", result);
    }

    assert!(result.is_ok());
    Ok(())
}
