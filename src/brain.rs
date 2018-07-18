// brain.rs handles all internal storage directory access
use batch::Batch;
use config::Config;
use errors::*;
use std::{fs::read_dir, path::PathBuf};

// This is my internal folder
// I want to be able to serialize/deserialize the contents
#[derive(Debug, PartialEq)]
pub struct Brain {
    pub batch: Batch,
}

impl Brain {
    // Always succeeds.  If no brain exists, makes a new one
    pub fn get(c: &Config) -> Result<Self> {
        let _dir = read_brain_dir(c)?;

        // if not, make a new one, write it, and return it
        let brain = Brain {
            batch: Batch::new(),
        };
        //write_brain_dir(&brain)?;
        Ok(brain)
    }
    // maybe have a len()?
}

// TODO add arguments to only modify/read in parts of the whole thing
// low prio because the numbers are pretty small

fn read_brain_dir(c: &Config) -> Result<(Brain)> {
    let path = &c.directory.path;
    // This should be:
    // brain/
    // |  hx/
    // |    *.html
    // |  blah.html
    // |  blah2.html
    // |  batch<DATETIME>.html
    // There will be a cleanup task (maybe as part of report() that will push everything to hx)
    let dir_listing: Vec<PathBuf> = read_dir(path)
        .chain_err(|| "Could not read brain!")?
        .map(|f| f.expect("Could not read brain entry").path())
        .collect();
    // need to grab the batch
    println!("Brain:\n{:#?}\n", dir_listing);
    // TEMPORARY until I finish this fn
    Ok(Brain {
        batch: Batch::new(),
    })
}

pub fn write_brain_dir(_b: &Brain) -> Result<()> {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_current_batch() {
        assert_eq!(
            Brain::get(&Config::default()).unwrap(),
            Brain {
                batch: Batch::new()
            }
        )
    }
}
