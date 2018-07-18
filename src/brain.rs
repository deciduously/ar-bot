// brain.rs handles all internal storage directory access
use batch::Batch;
use config::Config;
use errors::*;

// This is my internal folder
// I want to be able to serialize/deserialize the contents
#[derive(Debug)]
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
        write_brain_dir(&brain)?;
        Ok(brain)
    }
}

fn read_brain_dir(_c: &Config) -> Result<(Brain)> {
    unimplemented!()
}

pub fn write_brain_dir(_b: &Brain) -> Result<()> {
    unimplemented!()
}

pub fn get_current_batch(c: &Config) -> Result<(Batch)> {
    let brain = Brain::get(c)?;
    Ok(brain.batch)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic] // i wrote in unimplemented, so for now it should
    fn test_get_current_batch() {
        assert_eq!(get_current_batch(&Config::default()).unwrap(), Batch::new())
    }
}
