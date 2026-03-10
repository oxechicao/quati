use std::env::{self, VarError};
use std::path::{Path, PathBuf};

pub struct Env;
impl Env {
    pub fn get_env(name: &str) -> Result<String, VarError> {
        env::var(name)
    }

    pub fn get_ssh_key_path() -> PathBuf {
        Self::get_env("QUATI_SSH_KEY_PATH")
            .map(PathBuf::from)
            .unwrap_or_else(|_| {
                // Default path logic (Standard SSH location)
                let home = env::var("HOME").unwrap_or_else(|_| ".".into());
                Path::new(&home).join(".ssh").join("id_rsa")
            })
    }

    pub fn get_custom_local_git_host() -> Result<String, VarError> {
        Self::get_env("QUATI_CUSTOM_LOCAL_GIT_HOST")
    }

    pub fn get_custom_remote_git_host() -> Result<String, VarError> {
        Self::get_env("QUATI_CUSTOM_REMOTE_GIT_HOST")
    }

    pub fn get_gitmoji() -> bool {
        match Self::get_env("QUATI_GITMOJIS_ENABLED") {
            Ok(value) => value == "true",
            Err(_) => false,
        }
    }

    pub fn get_stage_all() -> bool {
        match Self::get_env("QUATI_STAGE_ALL") {
            Ok(value) => value == "true",
            Err(_) => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;
    use temp_env::{with_var, with_vars};

    #[test]
    #[serial]
    fn test_env_var() {
        let any_key = "any_key";
        with_vars(vec![(any_key, Some("any_value"))], || {
            let result = Ok("any_value".to_string());
            assert_eq!(Env::get_env(any_key), result);
        });
    }

    #[test]
    #[serial]
    fn test_get_ssh_key_path() {
        let result = PathBuf::from("/ssh/key/path");
        with_vars(vec![("QUATI_SSH_KEY_PATH", Some("/ssh/key/path"))], || {
            assert_eq!(Env::get_ssh_key_path(), result);
        })
    }

    #[test]
    #[serial]
    fn test_should_fallback_when_ssh_path_not_exists() {
        with_var("QUATI_SSH_KEY_PATH", None::<String>, || {
            temp_env::with_var("HOME", Some("/home/user"), || {
                let result = Env::get_ssh_key_path();
                let expected = PathBuf::from("/home/user/.ssh/id_rsa");
                assert_eq!(result, expected);
            })
        })
    }

    #[test]
    #[serial]
    fn test_should_fallback_to_root_when_home_not_exists() {
        with_var("QUATI_SSH_KEY_PATH", None::<String>, || {
            temp_env::with_var("HOME", None::<String>, || {
                let result = Env::get_ssh_key_path();
                let expected = PathBuf::from("./.ssh/id_rsa");
                assert_eq!(result, expected);
            })
        })
    }

    #[test]
    #[serial]
    fn test_should_return_local_host_when_env_defined() {
        let key_value = "localgit:";
        with_vars(
            vec![("QUATI_CUSTOM_LOCAL_GIT_HOST", Some(key_value))],
            || {
                let response = Env::get_custom_local_git_host().ok();
                assert_eq!(response.unwrap(), key_value);
            },
        )
    }

    #[test]
    #[serial]
    fn test_should_return_remote_host_when_defined() {
        let key_value = "localgit:";
        with_vars(
            vec![("QUATI_CUSTOM_REMOTE_GIT_HOST", Some(key_value))],
            || {
                let response = Env::get_custom_remote_git_host().ok();
                assert_eq!(response.unwrap(), key_value);
            },
        )
    }

    #[test]
    #[serial]
    fn test_should_return_error_when_local_host_env_not_defined() {
        with_var("QUATI_CUSTOM_LOCAL_GIT_HOST", None::<String>, || {
            assert!(Env::get_custom_local_git_host().is_err());
        })
    }

    #[test]
    #[serial]
    fn test_should_return_true_when_gitmoji_enabled() {
        with_vars(vec![("QUATI_GITMOJIS_ENABLED", Some("true"))], || {
            assert!(Env::get_gitmoji());
        })
    }

    #[test]
    #[serial]
    fn test_should_return_false_when_gitmoji_env_not_set() {
        with_var("QUATI_GITMOJIS_ENABLED", None::<String>, || {
            assert!(!Env::get_gitmoji());
        })
    }

    #[test]
    #[serial]
    fn test_should_return_true_when_stage_all() {
        with_vars(vec![("QUATI_STAGE_ALL", Some("true"))], || {
            assert!(Env::get_stage_all());
        })
    }

    #[test]
    #[serial]
    fn test_should_return_false_when_stage_all_env_not_set() {
        with_var("QUATI_STAGE_ALL", None::<String>, || {
            assert!(!Env::get_stage_all());
        })
    }
}
