use config::{Config, ConfigError, File};

#[derive(Debug)]
#[allow(unused)]
pub struct Settings {
    name: String,
    author: String,
    version: String,
    about: String,
}

impl Settings {
    pub fn new() -> Result<Config, ConfigError> {
        let config = Config::builder()
            .add_source(File::with_name("config.toml"))
            .build()?;

        return Ok(config);
    }
}
