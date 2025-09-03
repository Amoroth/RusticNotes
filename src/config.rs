use serde::{Deserialize};

#[derive(Deserialize)]
pub struct RusticConfig {
    pub notes_directory: String,
    pub editor: Option<String>,
}

fn default_config() -> RusticConfig {
    println!("Using default configuration.");
    RusticConfig {
        notes_directory: ".".to_string(),
        editor: None,
    }
}

pub fn get_config() -> RusticConfig {
    match std::fs::read_to_string("config.toml") {
        Ok(data) => toml::from_str(&data).unwrap_or_else(|_| default_config()),
        Err(_) => default_config(),
    }
}

// todo try to guess a default editor before returning None
