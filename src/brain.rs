// brain.rs handles all internal storage directory access
use batch::{Batch, Entry};
use config::Config;
use errors::*;
use regex::Regex;
use std::{
    fs::{create_dir, read_dir, remove_dir_all, File}, io::prelude::*, path::{Path, PathBuf},
    str::FromStr,
};
use util::*;

// This is my internal folder
// I want to be able to serialize/deserialize the contents
#[derive(Debug, PartialEq)]
pub struct Brain {
    pub batch: Batch,
    pub emails: Vec<Email>,
}

impl Brain {
    pub fn new() -> Self {
        Brain {
            batch: Batch::new(),
            emails: Vec::new(),
        }
    }
    pub fn add_entry(&mut self, input_p: &str) -> Result<()> {
        // Read the file given into contents
        println!("Brain::add_entry input: {}", input_p);
        let contents = file_contents_from_str_path(input_p)?;
        println!("Brian::add_entry: {} {}", input_p, contents);

        // add raw email to brain
        self.emails.push(Email::new(&contents, &input_p)?);

        // add entry to the batch
        self.batch.add_entry(Entry::from_str(&contents)?)?;

        Ok(())
    }

    // This just returns a prefilled Brain for testing purposes
    #[cfg(test)]
    fn test() -> Self {
        Brain {
            batch: Batch::test(),
            emails: vec![Email {
                filename: "TEMPDATE.html".into(),
                contents: TEST_COOL_STR.into(),
            }],
        }
    }

    // maybe have a len() returning the hwo many emails we have
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
    // Reads the brain dir into memory from the dir specified in config.  If no brain exists, makes a new one
    pub fn read_fs(&mut self) -> Result<()> {
        lazy_static! {
            static ref BATCH_RE: Regex = Regex::new(r"^batch-TEMPDATE.html").unwrap();
        }

        let path = &format!("{}/", self.config.directory.path);

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
        println!("{:#?}", dir_listing);

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
                // FIXME this is where the tests are crashing and I dont know why
                println!("Pushing email: {}", p_str);
                let contents = file_contents_from_str_path(p_str)?;
                emails.push(Email::new(&contents, p_str).chain_err(|| "Could not add email")?);
            }
        }

        // Read in the current batch
        // If none exists, make a new one
        let batch = if current_batch_p == "" {
            Batch::new()
        } else {
            Batch::from_str(&file_contents_from_str_path(current_batch_p)?)?
        };

        // Put together the brain and store it back in the context
        self.brain = Brain { batch, emails };
        println!("Brain:\n{:#?}\n", self.brain);

        Ok(())
    }
    // Writes the in-memory brain out to the filesystem
    pub fn write_fs(&self) -> Result<()> {
        let prefix = &format!("{}/", self.config.directory.path);
        let path = Path::new(prefix);

        // Start from scratch
        remove_dir_all(path).chain_err(|| "Could not clean Brain")?;
        create_dir(path).chain_err(|| "Could not write fresh Brain")?;

        // write the batch
        let date = "TEMPDATE";

        // FIXME this should be more explicit than a dot expansion
        let batch_filename = format!("{}batch-{}.html", prefix, date);
        let mut batch_file =
            File::create(&batch_filename).chain_err(|| "Could not create batch file")?;
        batch_file
            .write_all(format!("{:#?}", self.brain.batch).as_bytes()) // AND THIS NEEDS TO BE {}
            .chain_err(|| "Could not write to batch file")?;
        // Compression will be easy - just use as_compressed_bytes or something

        // write each email
        //
        for email in &self.brain.emails {
            let mut e_file =
                File::create(&email.filename).chain_err(|| "Could not create email file")?;
            e_file
                .write_all(email.contents.as_bytes())
                .chain_err(|| "Could not write to email file")?;
        }

        Ok(())
    }
    // Have Brain::add_entry insert it properly, then persist it to disk
    pub fn add_entry(&mut self, input_p: &str) -> Result<()> {
        println!("Context::add_entry: {}", input_p);
        self.brain.add_entry(&input_p)?;
        self.write_fs()?;
        Ok(())
    }
    // BE CAREFUL - empties itself out.  Potential data loss if you call write_fs() afterwards
    // this is for testing purposes only
    #[cfg(test)]
    pub fn clean(&mut self) {
        self.brain = Brain::new();
        self.write_fs().unwrap();
    }
}

#[derive(Debug, PartialEq)]
pub struct Email {
    pub filename: String,
    pub contents: String,
}

impl Email {
    pub fn new(path: &str, filename: &str) -> Result<Self> {
        Ok(Email {
            filename: filename.into(),
            contents: file_contents_from_str_path(path)?,
        })
    }
}

impl FromStr for Email {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        Ok(Email {
            filename: format!("TEMPDATE.html"),
            contents: s.into(),
        })
    }
}

// TODO add arguments to only modify/read in parts of the whole thing
// low prio because the numbers are pretty small
// e.g. Brain::write_email(), Brain::get_email(), Brain::write_batch(), Brain::get_batch()

#[cfg(test)]
mod tests {
    use super::*;
    use rand::{thread_rng, Rng};
    #[test]
    fn test_add_entry_brain() {
        // Start with fresh test dir
        println!("Preparing temp dir...");
        let test_dir = format!{"{}", thread_rng().gen::<u32>()};
        ::std::fs::remove_dir_all(&test_dir)
            .unwrap_or_else(|e| eprintln!("Failed to remove test dir: {}", e));
        ::std::fs::create_dir(&test_dir).unwrap();

        let mut test_context: Context = Context::initialize(Config::test(&test_dir)).unwrap();

        // add our sample email AFTER init
        let sample_p = &format!("{}/sample-email.html", test_dir);
        let mut file = ::std::fs::File::create(sample_p).unwrap();
        file.write_all(TEST_COOL_STR.as_bytes()).unwrap();

        test_context.brain.add_entry(sample_p).unwrap();
        let test_brain = test_context.brain;

        println!("Cleaning temp dir...");
        ::std::fs::remove_dir_all(test_dir)
            .unwrap_or_else(|e| eprintln!("Failed to remove test dir: {}", e));

        assert_eq!(test_brain, Brain::test())
    }

    #[test]
    fn test_read_fs() {
        // Start with fresh test dir
        println!("Preparing temp dir...");
        let test_dir = format!("{}", thread_rng().gen::<u32>());
        ::std::fs::remove_dir_all(&test_dir)
            .unwrap_or_else(|e| eprintln!("Failed to remove test dir: {}", e));
        ::std::fs::create_dir(&test_dir).unwrap();

        // save the test email to the temp dir
        let sample_p = &format!("{}/sample-email.html", test_dir);
        let mut file = ::std::fs::File::create(sample_p).unwrap();
        file.write_all(TEST_COOL_STR.as_bytes()).unwrap();

        // add the entry to the running test batch
        // it will read the entry during initialization
        let mut test_context: Context = Context::initialize(Config::test(&test_dir)).unwrap();
        //test_context.add_entry(sample_p).unwrap();

        // Clear memory, and re-read from fs
        //test_context.clean();
        //test_context.read_fs().unwrap();

        // grab result before cleanup
        let test_brain = test_context.brain;
        println!("Cleaning test dir...");
        ::std::fs::remove_dir_all(test_dir)
            .unwrap_or_else(|e| eprintln!("Failed to remove test dir: {}", e));

        // cleanup test dir before assert
        assert_eq!(test_brain, Brain::test())
    }
}
