// brain.rs handles all internal storage directory access
use batch::Batch;
use config::Config;
use errors::*;
use regex::Regex;
use std::{
    fs::{create_dir, read_dir, remove_dir_all, File}, io::prelude::*, path::{Path, PathBuf},
    str::FromStr,
};
use util::file_contents_from_str_path;

// This is my internal folder
// I want to be able to serialize/deserialize the contents
#[derive(Debug, PartialEq)]
pub struct Brain {
    pub batch: Batch,
    pub emails: Vec<Email>,
}

#[derive(Debug, PartialEq)]
pub struct Email {
    pub filename: String,
    pub contents: String,
}

impl Email {
    fn new(path: &str) -> Result<Self> {
        Ok(Email {
            filename: path.into(),
            contents: file_contents_from_str_path(path)?,
        })
    }
}

impl Brain {
    // Returns the current state of the brain - always succeeds.  If no brain exists, makes a new one
    pub fn get_all(c: &Config) -> Result<Self> {
        lazy_static! {
            static ref BATCH_RE: Regex = Regex::new(r"^batch\d+.html").unwrap();
        }

        let path = &c.directory.path;

        // If no path exists, create it.
        // std::fs::create_dir will return an error if the path exists
        if !Path::new(path).exists() {
            println!("No brain found!  Creating...");
            create_dir(path).chain_err(|| "Could not create brain dir")?;
        }

        // This should be:
        // brain/
        // |  hx/ - Do HX later
        // |    *.html
        // |  blah.html
        // |  blah2.html
        // |  batch<DATETIME>.html

        // There will be a cleanup task (maybe as part of report() that will push everything to hx)
        // dir_lisitng holds str paths of each file in Brain
        let dir_listing: Vec<PathBuf> = read_dir(path)
            .chain_err(|| "Could not read brain!")?
            .map(|f| f.expect("Could not read brain entry").path())
            .collect();

        // Grab the current batch
        // Save any emails
        let mut current_batch_p = "";
        let mut emails = Vec::new();
        for l in &dir_listing {
            let p_str = l.to_str().unwrap();
            if BATCH_RE.is_match(p_str) {
                current_batch_p = p_str;
            } else {
                // TODO check if its actually an email?
                // what do we do with non-expected files?
                emails.push(Email::new(p_str)?);
            }
        }

        // Read in the current batch
        // If none exists, make a new one
        let batch = if current_batch_p == "" {
            Batch::new()
        } else {
            Batch::from_str(&file_contents_from_str_path(current_batch_p)?)?
        };

        // Read in any emails

        let brain = Brain { batch, emails };

        println!("Brain:\n{:#?}\n", brain);
        Ok(brain)
    }

    // Writes a full brain out
    pub fn write_all(&self, c: &Config) -> Result<()> {
        let path = Path::new(&c.directory.path);

        // Start from scratch
        remove_dir_all(path).chain_err(|| "Could not clean Brain")?;
        create_dir(path).chain_err(|| "Could not write fresh Brain")?;

        // write the batch
        let date = "TEMPDATE";
        let batch_filename = format!("batch-{}", date);
        let mut batch_file =
            File::create(&batch_filename).chain_err(|| "Could not create batch file")?;
        batch_file
            .write_all(format!("{:#?}", self.batch).as_bytes())
            .chain_err(|| "Could not write to batch file")?;
        // Compression will be easy - just use as_compressed_bytes or something

        // write each email
        for email in &self.emails {
            let mut e_file =
                File::create(&email.filename).chain_err(|| "Could not create email file")?;
            e_file
                .write_all(email.contents.as_bytes())
                .chain_err(|| "Could not write to email file")?;
        }

        Ok(())
    }

    // maybe have a len() returning the hwo many emails we have
}

// TODO add arguments to only modify/read in parts of the whole thing
// low prio because the numbers are pretty small
// e.g. Brain::write_email(), Brain::get_email(), Brain::write_batch(), Brain::get_batch()

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_current_batch() {
        // TODO how do we write this so its guaranteed to be empty?
        // A seprate Config::testing()?
        assert_eq!(
            Brain::get_all(&Config::default()).unwrap(),
            Brain {
                batch: Batch::new(),
                emails: Vec::new(),
            }
        )
    }
}
