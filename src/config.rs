use errors::*;
use toml;

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

pub fn init_config(s: Option<&str>) -> Result<Config> {
    let config: Config = toml::from_str(&super::file_contents_from_str_path(s.unwrap_or(
        DEFAULT_CONFIG,
    ))?).chain_err(|| "Could not read config file")?;
    Ok(config)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init_config_default() {
        assert_eq!(init_config(None).unwrap(), Config::default())
    }
}
