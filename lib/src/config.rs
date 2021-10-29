use std::{
    fs::{create_dir_all, File},
    io::Write,
    path::{Path, PathBuf},
};

use config::{Config as AppConfig, ConfigError, Environment, File as ConfigFile};
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use toml::to_vec;

#[derive(Deserialize, Serialize)]
pub struct Database {
    pub path: String,
}

impl Default for Database {
    fn default() -> Self {
        let path: String = ProjectDirs::from("", "", "bookshelf")
            .unwrap()
            .config_dir()
            .join("db")
            .as_os_str()
            .to_str()
            .unwrap()
            .to_string();

        Database { path }
    }
}

#[derive(Default, Deserialize, Serialize)]
pub struct Config {
    pub db: Database,
}

impl Config {
    /// Returns a `Config` with settings from config.toml located in the
    /// OS's default config directory merged with the environment variables.
    /// If it is not created yet, initializes the config with default values.
    ///
    /// # Errors
    ///
    /// Returns `ConfigError` if either reading the config file or
    /// the environment has failed
    pub fn get_or_default() -> Result<Self, ConfigError> {
        match Self::exists() {
            true => Self::get(),
            false => Self::init(),
        }
    }

    /// Returns a `Config` with settings from config.toml located in the
    /// OS's default config directory merged with the environment variables
    ///
    /// # Errors
    ///
    /// Returns `ConfigError` if either reading the config file or
    /// the environment has failed
    fn get() -> Result<Self, ConfigError> {
        let config_path: PathBuf = ProjectDirs::from("", "", "bookshelf")
            .unwrap()
            .config_dir()
            .join("config.toml");

        let mut app_config: AppConfig = AppConfig::default();
        app_config.merge(ConfigFile::from(config_path)).unwrap();
        app_config
            .merge(Environment::with_prefix("BOOKSHELF"))
            .unwrap();

        Ok(app_config.try_into().unwrap())
    }

    /// Writes a default `Config` into config.toml located at the
    /// OS's default config directory, and returns it.
    ///
    /// # Panics
    ///
    /// Panics if there is some IO error
    fn init() -> Result<Self, ConfigError> {
        // Creating a config folder "bookshelf" if it doesn't exist yet
        let dirs: ProjectDirs = ProjectDirs::from("", "", "bookshelf").unwrap();
        let config_folder: &Path = dirs.config_dir();
        create_dir_all(config_folder).unwrap();

        // Creating a Config with default values
        let config: Config = Config::default();

        // Creating a config.toml file with default settings if it doesn't exist yet
        let config_file_path: PathBuf = config_folder.join("config.toml");
        let mut config_file: File = File::create(config_file_path).unwrap();

        // Writing the default Config to config.toml
        let config_toml = to_vec(&config).unwrap();
        config_file.write_all(&config_toml).unwrap();

        Ok(config)
    }

    /// Returns whether the config folder and config.toml exist
    fn exists() -> bool {
        ProjectDirs::from("", "", "bookshelf")
            .unwrap()
            .config_dir()
            .join("config.toml")
            .exists()
    }
}
