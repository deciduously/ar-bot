// brain.rs handles all internal storage directory access
use batch::Batch;
use config::Config;
use errors::*;
use std::{fs::{create_dir, read_dir}, path::{Path, PathBuf}};

// This is my internal folder
// I want to be able to serialize/deserialize the contents
#[derive(Debug, PartialEq)]
pub struct Brain {
    pub batch: Batch,
    pub emails: Vec<String>,
}

impl Brain {
    // Always succeeds.  If no brain exists, makes a new one
    pub fn get(c: &Config) -> Result<Self> {
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
        let dir_listing: Vec<PathBuf> = read_dir(path)
            .chain_err(|| "Could not read brain!")?
            .map(|f| f.expect("Could not read brain entry").path())
            .collect();
        // Grab the batch.  If it doesn't exist make a new one
        println!("Brain:\n{:#?}\n", dir_listing);
        // TEMPORARY until I finish this fn
        Ok(Brain {
            batch: Batch::new(),
            emails: Vec::new(),
        })
    }
    // maybe have a len()?
}

// TODO add arguments to only modify/read in parts of the whole thing
// low prio because the numbers are pretty small

pub fn write_brain_dir(_b: &Brain) -> Result<()> {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_current_batch() {
        assert_eq!(
            Brain::get(&Config::default()),
            Brain {
                batch: Batch::new(),
                emails: Vec::new(),
            }
        )
    }
}
