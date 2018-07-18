// config.rs handles loading and eventually writing to the app configuration
use errors::*;
use std::fmt;
use toml;
use util::file_contents_from_str_path;

static DEFAULT_CONFIG: &'static str = "Bot.toml";

// Eventually, allow for config manipulation via commandline
// Leaving Serialize in here for now

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct Config {
    directory: Directory,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct Directory {
    pub compressed: bool,
    pub path: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            directory: Directory {
                compressed: false,
                path: "./brain/".into(),
            },
        }
    }
}

impl fmt::Display for Config {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(
            f,
            "Ar-Bot Configuration:\n* Directory Settings:\n* * {}",
            self.directory
        )
    }
}

impl fmt::Display for Directory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut compressed_string = String::new();
        if !self.compressed {
            compressed_string.push_str("not ")
        }
        compressed_string.push_str("using");
        write!(
            f,
            "Brain path: {} - {} compression",
            self.path, compressed_string
        )
    }
}

pub fn init_config(s: Option<&str>) -> Result<Config> {
    let config: Config = toml::from_str(&file_contents_from_str_path(s.unwrap_or(DEFAULT_CONFIG))?)
        .chain_err(|| "Could not read config file")?;
    Ok(config)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init_config_default() {
        assert_eq!(init_config(None).unwrap(), Config::default())
    }
    #[test]
    fn test_init_config_alterate_file() {
        assert_eq!(
            init_config(Some("Alternate.toml")).unwrap(),
            Config {
                directory: Directory {
                    compressed: true,
                    path: "./storage/".into(),
                }
            }
        )
    }
}
