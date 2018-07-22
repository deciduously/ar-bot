// brain.rs handles all internal storage directory access
use config::Config;
use email::RawEmail;
use errors::*;
use regex::Regex;
use std::{
    fmt, fs::{create_dir, read_dir}, path::PathBuf,
};
use util::*;

// This is my internal folder
// I want to be able to serialize/deserialize the contents
#[derive(Debug)]
pub struct Brain {
    pub emails: Vec<RawEmail>,
}

impl Brain {
    pub fn new() -> Self {
        Brain { emails: Vec::new() }
    }
    // maybe have a len() returning the hwo many emails we have
}

impl fmt::Display for Brain {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut emails = String::new();
        for email in &self.emails {
            emails.push_str(&format!("\r\n{}\r\n", email));
        }
        write!(f, "emails: {}", emails)
    }
}

// This is the running app state.  Is State a better name?
#[derive(Debug)]
pub struct Context {
    pub config: Config,
    pub brain: Brain,
}

impl Context {
    // Take ownership over the fresh ones passed in
    pub fn initialize(config: Config) -> Result<Self> {
        let mut ctx = Context {
            config,
            brain: Brain::new(),
        };

        ctx.read_fs()?;
        Ok(ctx)
    }

    pub fn hx_path(&self) -> PathBuf {
        let mut ret = self.config.directory.path.clone();
        ret.push("hx");
        ret
    }

    // Reads the brain dir into memory from the dir specified in config.  If no brain exists, makes a new one
    // this is still prpbably all relevant
    // TODO proper Path usage.  Lets start here.
    pub fn read_fs(&mut self) -> Result<()> {
        lazy_static! {
            static ref BATCH_RE: Regex = Regex::new(r"^batch-TEMPDATE.html").unwrap();
        }

        let brain_path = &self.config.directory.path;

        // If no path exists, create it.
        // std::fs::create_dir will return an error if the path exists
        if !brain_path.exists() {
            warn!("No brain found!  Creating...");
            create_dir(brain_path).chain_err(|| "Could not create brain dir")?;
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
        let dir_listing: Vec<PathBuf> = read_dir(brain_path)
            .chain_err(|| "Could not read brain!")?
            .map(|f| f.expect("Could not read brain entry").path())
            .collect();

        // Grab the current batch
        // Save any emails
        let mut emails = Vec::new();
        for l in &dir_listing {
            let p_str = l.to_str().unwrap();
            if BATCH_RE.is_match(p_str) {
                continue;
            } else {
                // TODO check if its actually an email?
                // what do we do with non-expected files?
                // FIXME this is where the tests are crashing and I dont know why
                info!("READ: {}", p_str);
                let contents = file_contents_from_str_path(p_str)?;
                emails.push(RawEmail::new(p_str, &contents).chain_err(|| "Could not add email")?);
            }
        }

        // Put together the brain and store it back in the context
        self.brain = Brain { emails };
        debug!("Brain: {}", self.brain);

        Ok(())
    }
    // Writes the in-memory brain out to the filesystem
    // This is going to change - no more full-file stuff.
    // The "brain" concept probably needs to be renamed.
    // We're just writing a digest and copying files into hx/
    // some of this will be useful for wirting the digest
    //    pub fn write_fs(&self) -> Result<()> {
    //        let prefix = &format!("{:?}/", self.config.directory.path);  // TODO this is bad
    //        let path = Path::new(prefix);
    //
    //        // Start from scratch
    //        remove_dir_all(path).chain_err(|| "Could not clean Brain")?;
    //        create_dir(path).chain_err(|| "Could not write fresh Brain")?;
    //
    //        // write the batch
    //        let date = "TEMPDATE";
    //
    //        // FIXME this should be more explicit than a dot expansion
    //        let batch_filename = format!("{}batch-{}.html", prefix, date);
    //        let mut batch_file =
    //            File::create(&batch_filename).chain_err(|| "Could not create batch file")?;
    //        batch_file
    //            .write_all(format!("{}", self.brain.batch).as_bytes())
    //           .chain_err(|| "Could not write to batch file")?;
    //        // Compression will be easy - just use as_compressed_bytes or something
    //
    //        // write each email
    //        //
    //       for email in &self.brain.emails {
    //            let mut e_file =
    //                File::create(&email.filename).chain_err(|| "Could not create email file")?;
    //            e_file
    //                .write_all(email.contents.as_bytes())
    //                .chain_err(|| "Could not write to email file")?;
    //        }
    //
    //        Ok(())
    //    }
}

// TODO add arguments to only modify/read in parts of the whole thing
// low prio because the numbers are pretty small
// e.g. Brain::write_email(), Brain::get_email(), Brain::write_batch(), Brain::get_batch()

#[cfg(test)]
mod tests {
    //use super::*;

    //#[test]
    //fn test_initialize_empty() {

    //    }

    //#[test]
    //fn test_initialize_not_empty() {

    //}
}
