use std::env::{self, VarError};
use std::path::{Path, PathBuf};

pub fn get_env(name: &str) -> Result<String, VarError> {
    let _ = dotenv::dotenv();
    env::var(name)
}

pub fn get_ssh_key_path() -> PathBuf {
    get_env("QUATI_SSH_KEY_PATH")
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            // Default path logic (Standard SSH location)
            let home = env::var("HOME").unwrap_or_else(|_| ".".into());
            Path::new(&home).join(".ssh").join("id_rsa")
        })
}

pub fn get_custom_local_git_host() -> Result<String, String> {
    let _ = dotenv::dotenv();
    match get_env("QUATI_CUSTOM_LOCAL_GIT_HOST not set") {
        Ok(value) => Ok(value),
        Err(_) => Ok("No custom remote defined".to_string()),
    }
}

pub fn get_custom_remote_git_host() -> Result<String, String> {
    match get_env("QUATI_CUSTOM_REMOTE_GIT_HOST") {
        Ok(value) => Ok(value),
        Err(_) => Ok("git@github.com:".to_string()),
    }
}

pub fn get_gitmoji() -> bool {
    match get_env("QUATI_GITMOJI_ENABLED") {
        Ok(value) => value == "true",
        Err(_) => false,
    }
}

pub fn get_stage_all() -> bool {
    match get_env("QUATI_STAGE_ALL") {
        Ok(value) => value == "true",
        Err(_) => false,
    }
}
