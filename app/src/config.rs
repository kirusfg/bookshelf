use config::{
    Config as AppConfig, ConfigError, Environment, File as ConfigFile,
};
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::{
    fs::{create_dir_all, File},
    io::Write,
    path::PathBuf,
};
use toml::to_vec;

/// The configuration for the app.
#[derive(Deserialize, Serialize)]
pub struct Config {
    /// The path to a file for the Shelf to be saved to.
    pub db: PathBuf,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            db: Self::default_config_dir().join("db"),
        }
    }
}

impl Config {
    /// A helper function, returns the PathBuf to the default config directory.
    pub fn default_config_dir() -> PathBuf {
        ProjectDirs::from("com", "kirusfg", "bookshelf")
            .unwrap()
            .config_dir()
            .to_path_buf()
    }

    /// Returns a `Config` with settings from config.toml located in the
    /// OS's default config directory merged with the environment variables.
    /// If it is not created yet, initializes the config with default values.
    ///
    /// # Errors
    ///
    /// Returns `ConfigError` if either reading the config file or
    /// the environment has failed.
    pub fn get_or_default() -> Result<Self, ConfigError> {
        match Self::exists() {
            true => Self::get(),
            false => Self::init(),
        }
    }

    /// Returns a `Config` with settings from config.toml located in the
    /// OS's default config directory merged with the environment variables.
    ///
    /// # Errors
    ///
    /// Returns `ConfigError` if either reading the config file or
    /// the environment has failed.
    pub fn get() -> Result<Self, ConfigError> {
        AppConfig::builder()
            .add_source(ConfigFile::from(
                Self::default_config_dir().join("config.toml"),
            ))
            .add_source(Environment::with_prefix("BOOKSHELF"))
            .build()
            .unwrap()
            .try_deserialize::<Config>()
    }

    /// Writes a default `Config` into config.toml located at the
    /// OS's default config directory, and returns it.
    ///
    /// # Errors
    ///
    /// Returns an error if either config directory or file could not be
    /// created.
    pub fn init() -> Result<Self, ConfigError> {
        if create_dir_all(Self::default_config_dir()).is_err() {
            return Err(ConfigError::Message(
                "Couldn't create the config directory".to_string(),
            ));
        }

        let default_config = Config::default();
        let mut config_file =
            File::create(Self::default_config_dir().join("config.toml"))
                .unwrap();

        // Writing the default Config to config.toml
        let config_toml = to_vec(&Config::default()).unwrap();
        config_file.write_all(&config_toml).unwrap();

        Ok(default_config)
    }

    /// Returns whether the config folder and config.toml exist.
    pub fn exists() -> bool {
        Self::default_config_dir().join("config.toml").exists()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_config() {
        if Config::init().is_ok() {
            assert!(Config::exists());
        }
    }
}
