use std::path::PathBuf;

use config::{Config, ConfigError, Environment, File};
use directories::ProjectDirs;

/// Returns a `Config` from the OS's default config directory
///
/// # Errors
///
/// Returns `ConfigError` if either reading the config file or
/// the environment has failed
pub fn get_config() -> Result<Config, ConfigError> {
    let config_path: PathBuf = ProjectDirs::from("", "", "bookshelf")
        .unwrap()
        .config_dir()
        .join("config.toml");

    let mut config: Config = Config::default();
    config.merge(File::from(config_path))?;
    config.merge(Environment::with_prefix("BOOKSHELF"))?;

    Ok(config)
}
