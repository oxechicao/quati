use std::env;
use std::path::{Path, PathBuf};

pub fn get_ssh_key_path() -> PathBuf {
    // Load .env file if it exists
    let _ = dotenv::dotenv();

    env::var("QUATI_SSH_KEY_PATH")
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            // Default path logic (Standard SSH location)
            let home = env::var("HOME").unwrap_or_else(|_| ".".into());
            Path::new(&home).join(".ssh").join("id_rsa")
        })
}
