// brain.rs handles all internal storage directory access
use batch::Batch;
use errors::*;

pub fn get_current_batch() -> Result<(Batch)> {
    Ok(Batch::new())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_current_batch() {
        assert_eq!(get_current_batch().unwrap(), Batch::new())
    }
}
