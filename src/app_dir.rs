use anyhow::{Context, Result};
use std::path::PathBuf;

pub(crate) fn config_file() -> Option<PathBuf> {
    let mut a = dirs::config_dir()?;
    a.push("foro.json");
    Some(a)
}

pub(crate) fn cache_dir() -> Option<PathBuf> {
    let mut a = dirs::cache_dir()?;
    a.push("foro");
    Some(a)
}

#[cfg(target_os = "linux")]
pub(crate) fn socket_dir() -> Option<PathBuf> {
    let mut a = dirs::runtime_dir()?;
    a.push("foro");
    Some(a)
}

#[cfg(target_os = "macos")]
pub(crate) fn socket_dir() -> Option<PathBuf> {
    // fixme: this is not best place
    //   /var/run/ have permission problem
    let mut a = dirs::config_dir()?;
    a.push("foro-socket-tmp");
    Some(a)
}

#[cfg(windows)]
pub(crate) fn socket_dir() -> Option<PathBuf> {
    // fixme: this is not best place
    let mut a = dirs::config_dir()?;
    a.push("foro-socket-tmp");
    Some(a)
}

pub(crate) fn log_dir() -> Option<PathBuf> {
    // is this correct?
    let mut a = socket_dir()?;
    a.push("log");
    Some(a)
}

#[allow(unused)]
pub(crate) fn config_file_res() -> Result<PathBuf> {
    config_file().context("Failed to get default config file")
}

pub(crate) fn cache_dir_res() -> Result<PathBuf> {
    cache_dir().context("Failed to get default cache dir")
}

pub(crate) fn socket_dir_res() -> Result<PathBuf> {
    socket_dir().context("Failed to get default socket dir")
}

pub(crate) fn log_dir_res() -> Result<PathBuf> {
    log_dir().context("Failed to get default log dir")
}
