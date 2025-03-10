use crate::app_dir::{AppDirResolver, DefaultAppDirResolver};
use crate::config::load_file;
use crate::config::model::Config;
use crate::debug_long;
use anyhow::{anyhow, Context, Result};
use log::{debug, info};
use std::fs;
use std::io::Read;
use std::path::PathBuf;

// functions that manually inject resolvers
// ----------------------------------------

pub(crate) fn get_or_create_default_config_with<R: AppDirResolver>(
    resolver: &R,
) -> Option<PathBuf> {
    let config_path = resolver.config_file()?;

    if !config_path.exists() {
        debug!("try create default config file: {:?}", config_path);

        fs::DirBuilder::new()
            .recursive(true)
            .create(&config_path.parent()?)
            .ok()?;

        let default_config = include_str!("default_config.json");

        fs::write(&config_path, &default_config).ok()?;

        info!("created default config file: {:?}", config_path);
        info!("content: {:?}", default_config);
    }

    Some(config_path)
}

pub(crate) fn load_config_and_cache_with<R: AppDirResolver>(
    resolver: &R,
    given_config_file: &Option<PathBuf>,
    given_cache_dir: &Option<PathBuf>,
) -> Result<(Config, PathBuf)> {
    let config_file = given_config_file
        .clone()
        .or_else(|| get_or_create_default_config_with(resolver))
        .context("Could not get config directory")?;

    let config = load_file(&config_file)
        .with_context(|| format!("Failed to load config file ({:?})", &config_file))?;

    let cache_dir = given_cache_dir
        .clone()
        .or(config.cache_dir.clone())
        .or_else(|| resolver.cache_dir())
        .context("Failed to get cache directory")?;

    debug!("config file: {:?}", &config_file);
    debug_long!("config: {:?}", &config);
    debug!("cache dir: {:?}", &cache_dir);

    Ok((config, cache_dir))
}

pub(crate) fn load_config_and_socket_with<R: AppDirResolver>(
    resolver: &R,
    given_config_file: &Option<PathBuf>,
    given_socket_dir: &Option<PathBuf>,
) -> Result<(Config, PathBuf)> {
    let config_file = given_config_file
        .clone()
        .or_else(|| get_or_create_default_config_with(resolver))
        .context("Failed to get config directory")?;

    let config = load_file(&config_file)
        .with_context(|| format!("Failed to load config file ({:?})", &config_file))?;

    let socket_dir = given_socket_dir
        .clone()
        .or(config.socket_dir.clone())
        .or_else(|| resolver.socket_dir())
        .context("Failed to get socket directory")?;

    debug!("config file: {:?}", &config_file);
    debug_long!("config: {:?}", &config);
    debug!("socket dir: {:?}", &socket_dir);

    Ok((config, socket_dir))
}

pub(crate) fn load_paths_with<R: AppDirResolver>(
    resolver: &R,
    given_config_file: &Option<PathBuf>,
    given_cache_dir: &Option<PathBuf>,
    given_socket_dir: &Option<PathBuf>,
) -> Result<(PathBuf, PathBuf, PathBuf)> {
    let config_file = given_config_file
        .clone()
        .or_else(|| get_or_create_default_config_with(resolver))
        .context("Failed to get config directory")?;

    let config = load_file(&config_file)
        .with_context(|| format!("Failed to load config file ({:?})", &config_file))?;

    let cache_dir = given_cache_dir
        .clone()
        .or(config.cache_dir.clone())
        .or_else(|| resolver.cache_dir())
        .context("Failed to get cache directory")?;

    let socket_dir = given_socket_dir
        .clone()
        .or(config.socket_dir.clone())
        .or_else(|| resolver.socket_dir())
        .context("Failed to get socket directory")?;

    Ok((config_file, cache_dir, socket_dir))
}

// functions that use the default resolver
// ----------------------------------------

pub(crate) fn get_or_create_default_config() -> Option<PathBuf> {
    get_or_create_default_config_with(&DefaultAppDirResolver {})
}

pub(crate) fn load_config_and_cache(
    given_config_file: &Option<PathBuf>,
    given_cache_dir: &Option<PathBuf>,
) -> Result<(Config, PathBuf)> {
    load_config_and_cache_with(
        &DefaultAppDirResolver {},
        given_config_file,
        given_cache_dir,
    )
}

pub(crate) fn load_config_and_socket(
    given_config_file: &Option<PathBuf>,
    given_socket_dir: &Option<PathBuf>,
) -> Result<(Config, PathBuf)> {
    load_config_and_socket_with(
        &DefaultAppDirResolver {},
        given_config_file,
        given_socket_dir,
    )
}

pub(crate) fn load_paths(
    given_config_file: &Option<PathBuf>,
    given_cache_dir: &Option<PathBuf>,
    given_socket_dir: &Option<PathBuf>,
) -> Result<(PathBuf, PathBuf, PathBuf)> {
    load_paths_with(
        &DefaultAppDirResolver {},
        given_config_file,
        given_cache_dir,
        given_socket_dir,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app_dir::AppDirResolver;
    use anyhow::Result;
    use std::fs;
    use std::path::PathBuf;
    use tempfile::tempdir;

    /// A mock resolver that returns predefined paths.
    struct MockResolver {
        config_path: Option<PathBuf>,
        cache_path: Option<PathBuf>,
        socket_path: Option<PathBuf>,
        log_path: Option<PathBuf>,
    }

    impl AppDirResolver for MockResolver {
        fn config_file(&self) -> Option<PathBuf> {
            self.config_path.clone()
        }

        fn cache_dir(&self) -> Option<PathBuf> {
            self.cache_path.clone()
        }

        fn socket_dir(&self) -> Option<PathBuf> {
            self.socket_path.clone()
        }

        fn log_dir(&self) -> Option<PathBuf> {
            self.log_path.clone()
        }
    }

    /// Returns the content of default_config.json for comparison.
    fn default_config_str() -> &'static str {
        include_str!("default_config.json")
    }

    #[test]
    fn test_get_or_create_default_config_with_creates_file_if_not_exists() -> Result<()> {
        let temp_dir = tempdir()?;
        let config_file_path = temp_dir.path().join("foro.json");

        let resolver = MockResolver {
            config_path: Some(config_file_path.clone()),
            cache_path: None,
            socket_path: None,
            log_path: None,
        };

        // Should create the default config file if it doesn't exist.
        let returned_path =
            get_or_create_default_config_with(&resolver).expect("Expected a valid path.");

        assert_eq!(returned_path, config_file_path);

        // Check if the file was actually created with default content.
        let content = fs::read_to_string(&config_file_path)?;
        assert_eq!(content, default_config_str());

        Ok(())
    }

    #[test]
    fn test_get_or_create_default_config_with_does_not_overwrite_if_exists() -> Result<()> {
        let temp_dir = tempdir()?;
        let config_file_path = temp_dir.path().join("foro.json");

        // Write a different initial content.
        let initial_content = r#"{"rules":[]}"#;
        fs::write(&config_file_path, initial_content)?;

        let resolver = MockResolver {
            config_path: Some(config_file_path.clone()),
            cache_path: None,
            socket_path: None,
            log_path: None,
        };

        // Should not overwrite existing content.
        let returned_path =
            get_or_create_default_config_with(&resolver).expect("Expected a valid path.");
        assert_eq!(returned_path, config_file_path);

        let content_after = fs::read_to_string(&config_file_path)?;
        assert_eq!(content_after, initial_content);

        Ok(())
    }

    #[test]
    fn test_load_config_and_cache_with_given_paths() -> Result<()> {
        let temp_dir = tempdir()?;
        let config_file_path = temp_dir.path().join("custom_config.json");
        let custom_cache_dir = temp_dir.path().join("my_cache");

        // This config file explicitly sets a cache_dir inside the JSON.
        let config_json = r#"
        {
          "rules": [
            {
              "on": ".rs",
              "cmd": "https://example.com/rust.dllpack"
            }
          ],
          "cache_dir": "config_defined_cache_dir", 
          "socket_dir": "config_defined_socket_dir"
        }
        "#;
        fs::write(&config_file_path, config_json)?;

        let resolver = MockResolver {
            config_path: Some(config_file_path.clone()),
            cache_path: None,
            socket_path: None,
            log_path: None,
        };

        let given_config_file = Some(config_file_path.clone());
        let given_cache_dir = Some(custom_cache_dir.clone());

        // User-provided cache_dir should take precedence.
        let (loaded_config, loaded_cache_dir) =
            load_config_and_cache_with(&resolver, &given_config_file, &given_cache_dir)?;

        assert_eq!(loaded_cache_dir, custom_cache_dir);
        assert_eq!(loaded_config.rules.len(), 1);
        assert!(loaded_config.rules[0].some_cmd.is_pure());

        Ok(())
    }

    #[test]
    fn test_load_config_and_cache_with_no_given_paths_uses_resolver() -> Result<()> {
        let temp_dir = tempdir()?;
        let config_file_path = temp_dir.path().join("foro.json");
        let cache_dir_path = temp_dir.path().join("cache_dir_from_resolver");

        // The resolver should return paths, but the config doesn't exist yet.
        let resolver = MockResolver {
            config_path: Some(config_file_path.clone()),
            cache_path: Some(cache_dir_path.clone()),
            socket_path: None,
            log_path: None,
        };

        // This should create the default config and use resolver's cache_dir.
        let (config, used_cache_dir) = load_config_and_cache_with(&resolver, &None, &None)?;

        let written_config = fs::read_to_string(&config_file_path)?;
        assert_eq!(written_config, default_config_str());
        assert_eq!(used_cache_dir, cache_dir_path);
        assert_eq!(config.rules.len(), 5);

        Ok(())
    }

    #[test]
    fn test_load_config_and_socket_with_custom_values() -> Result<()> {
        let temp_dir = tempdir()?;
        let config_file_path = temp_dir.path().join("config.json");
        let socket_dir_path = temp_dir.path().join("custom_socket_dir");

        // This config sets a socket_dir, but we will override it.
        let config_json = r#"
        {
            "rules": [],
            "cache_dir": "/ignore_this",
            "socket_dir": "config_defined_socket_dir"
        }
        "#;
        fs::write(&config_file_path, config_json)?;

        let resolver = MockResolver {
            config_path: Some(config_file_path.clone()),
            cache_path: None,
            socket_path: None,
            log_path: None,
        };

        let (config, used_socket_dir) = load_config_and_socket_with(
            &resolver,
            &Some(config_file_path),
            &Some(socket_dir_path.clone()),
        )?;

        assert!(config.rules.is_empty());
        assert_eq!(used_socket_dir, socket_dir_path);

        Ok(())
    }

    #[test]
    fn test_load_paths_with_fallback_order() -> Result<()> {
        let temp_dir = tempdir()?;
        let config_file_path = temp_dir.path().join("some_config.json");

        let config_json = r#"
        {
            "rules": [],
            "cache_dir": "/from_config_cache",
            "socket_dir": "/from_config_socket"
        }
        "#;
        fs::write(&config_file_path, config_json)?;

        let resolver_cache = temp_dir.path().join("resolver_cache");
        let resolver_socket = temp_dir.path().join("resolver_socket");

        let resolver = MockResolver {
            config_path: Some(config_file_path.clone()),
            cache_path: Some(resolver_cache.clone()),
            socket_path: Some(resolver_socket.clone()),
            log_path: None,
        };

        // If config_file is Some(...), it should load from it.
        let (actual_config_file, actual_cache, actual_socket) =
            load_paths_with(&resolver, &Some(config_file_path.clone()), &None, &None)?;

        assert_eq!(actual_config_file, config_file_path);
        assert_eq!(actual_cache, PathBuf::from("/from_config_cache"));
        assert_eq!(actual_socket, PathBuf::from("/from_config_socket"));

        // If config_file is None, it creates the default config if none is found.
        fs::remove_file(&config_file_path)?; // Remove it to force creation.
        let (cf, _, _) = load_paths_with(&resolver, &None, &None, &None)?;
        let default_content = fs::read_to_string(&config_file_path)?;
        assert_eq!(default_content, default_config_str());
        assert_eq!(cf, config_file_path);

        Ok(())
    }

    #[test]
    fn test_error_when_config_file_not_found_in_resolver() {
        let resolver = MockResolver {
            config_path: None,
            cache_path: None,
            socket_path: None,
            log_path: None,
        };

        // This should return None, since the resolver gives no path.
        let result = get_or_create_default_config_with(&resolver);
        assert!(result.is_none());

        // load_config_and_cache_with should fail in this situation.
        let err = load_config_and_cache_with(&resolver, &None, &None).unwrap_err();
        assert!(
            err.to_string().contains("Could not get config directory"),
            "Expected an error about missing config directory."
        );
    }
}
