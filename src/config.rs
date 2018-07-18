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
    pub config_path: Option<String>, // Workaround so we can fill it in during initialiation without an intermediate struct
    pub directory: Directory,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct Directory {
    pub compressed: bool,
    pub path: String,
}

impl Config {
    fn add_config_path(&mut self, s: &str) -> Result<()> {
        self.config_path = Some(s.into());
        Ok(())
    }
    // TODO add more self mutations here, use Clap subcommands to provide easy access
}

impl Default for Config {
    fn default() -> Self {
        Config {
            config_path: Some("Bot.toml".into()),
            directory: Directory {
                compressed: false,
                path: "./brain/".into(),
            },
        }
    }
}

impl fmt::Display for Config {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let c_p = self.config_path.clone().unwrap_or_else(|| "None given".into());
        write!(
            f,
            "Ar-Bot Configuration:\n* Configuration file path: {}\n* Directory Settings:\n* * {}",
            c_p, self.directory,
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
    let mut config: Config = toml::from_str(&file_contents_from_str_path(s.unwrap_or(
        DEFAULT_CONFIG,
    ))?).chain_err(|| "Could not read config file")?;
    config.add_config_path(s.unwrap_or(DEFAULT_CONFIG))?;
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
                config_path: Some("Alternate.toml".into()),
                directory: Directory {
                    compressed: true,
                    path: "./storage/".into(),
                }
            }
        )
    }
}
