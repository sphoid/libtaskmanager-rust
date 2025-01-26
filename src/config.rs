use std::error::Error;
use std::path::Path;
use std::io::BufReader;
use serde::{Deserialize, Serialize};

const CONFIG_FILE: &str = "config.json";

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum PersistenceMode {
	JSON,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
	pub persistence_mode: PersistenceMode,
}

impl Config {
	pub fn default() -> Self {
		Self {
			persistence_mode: PersistenceMode::JSON,
		}
	}
}

pub fn load_config() -> Result<Config, Box<dyn Error>> {
	let config_path = Path::new(CONFIG_FILE);
	if !config_path.exists() {
		return Ok(Config::default());
	}

	let file = std::fs::File::open(config_path)?;
	let reader = BufReader::new(file);
	let config: Config = serde_json::from_reader(reader)?;

	Ok(config)
}
