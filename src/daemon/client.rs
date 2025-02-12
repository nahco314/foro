use crate::cli::GlobalOptions;
use crate::daemon::interface::{
    DaemonBulkFormatResponse, DaemonCommandPayload, DaemonCommands, DaemonFormatResponse,
    DaemonPureFormatResponse, DaemonResponse, DaemonSocketPath,
};
use crate::daemon::uds::UnixStream;
use crate::process_utils::{get_start_time, is_alive};
use anyhow::{anyhow, Context, Result};
use log::{debug, info};
use std::env::current_dir;
use std::io::{ErrorKind, Write};
use std::time::Duration;

fn parse_info(info_str: &str) -> Option<(u32, u64)> {
    let parts: Vec<&str> = info_str.split(',').collect();

    if parts.len() != 2 {
        return None;
    }

    let pid = parts[0].parse().ok()?;
    let start_time = parts[1].parse().ok()?;

    Some((pid, start_time))
}

/// Check if the daemon is alive.
///
/// It works as follows:
/// 1. Reads the daemon's pid and start time from the daemon's information file
///    (which is located in the same position as the socket, etc.)
/// 2. Asks the OS for information about the process with that pid
/// 3. Checks that the process is currently alive and that the start time matches
///    (because after a process has ended, another process with the same pid may start)
///
/// This is similar to [ping], but this function only determines whether the process is alive,
/// whereas ping actually communicates and makes a determination.
/// In other words, ping is more accurate but also slower.
pub fn daemon_is_alive(socket: &DaemonSocketPath) -> Result<bool> {
    // note: don't call path.exits()
    //       because we can reduce the number of system calls and speed up (a little bit!)
    let content = match std::fs::read_to_string(&socket.info_path) {
        Ok(content) => content,
        Err(err) if err.kind() == ErrorKind::NotFound => return Ok(false),
        Err(err) => return Err(err.into()),
    };
    let (pid, start_time) = parse_info(&content).context("Failed to parse daemon info")?;

    if !is_alive(pid) {
        return Ok(false);
    }

    if get_start_time(pid)? != start_time {
        return Ok(false);
    }

    Ok(true)
}

/// Send a ping to the daemon.
///
/// See also [daemon_is_alive].
pub fn ping(socket: &DaemonSocketPath) -> Result<bool> {
    match UnixStream::connect(&socket.socket_path) {
        Ok(stream) => {
            match run_command_inner(
                DaemonCommands::Ping,
                GlobalOptions {
                    config_file: None,
                    cache_dir: None,
                    socket_dir: None,
                    no_cache: false,
                    no_long_log: false,
                },
                stream,
                Some(Duration::from_secs(1)),
            ) {
                Ok(DaemonResponse::Pong(_)) => Ok(true),
                Ok(_) => Ok(false),
                Err(err) => {
                    let was_timed_out = err
                        .downcast_ref::<serde_json::Error>()
                        .and_then(|e| e.io_error_kind())
                        .is_some_and(|k| k == ErrorKind::WouldBlock);

                    if was_timed_out {
                        info!("ping timed out");
                        Ok(false)
                    } else {
                        Err(err.into())
                    }
                }
            }
        }
        Err(err) if err.kind() == ErrorKind::ConnectionRefused => Ok(false),
        Err(err) if err.kind() == ErrorKind::NotFound => Ok(false),
        Err(err) => Err(err.into()),
    }
}

fn run_command_inner(
    command: DaemonCommands,
    global_options: GlobalOptions,
    mut stream: UnixStream,
    timeout: Option<Duration>,
) -> Result<DaemonResponse> {
    let buf = serde_json::to_vec(&DaemonCommandPayload {
        command,
        current_dir: current_dir()?,
        global_options,
    })?;
    stream.write_all(&buf)?;

    debug!("Sent command");

    stream.shutdown(std::net::Shutdown::Write)?;

    stream.set_read_timeout(timeout)?;

    let res = serde_json::from_reader(stream)?;

    debug!("Received response");

    Ok(res)
}

/// Run a command with the daemon.
///
/// This function sends a command to the daemon which is running on the given socket,
/// and outputs the result.
pub fn run_command(
    command: DaemonCommands,
    global_options: GlobalOptions,
    socket: &DaemonSocketPath,
    check_alive: bool,
) -> Result<()> {
    if check_alive && !daemon_is_alive(&socket)? {
        match command {
            DaemonCommands::Stop => {
                // in rare cases, daemon_is_alive return false, but the process may still be alive
                if !ping(&socket)? {
                    info!("Daemon is not running");
                    return Ok(());
                }
            }
            _ => {
                return Err(anyhow!("Daemon is not running!"));
            }
        }
    }

    let stream = UnixStream::connect(&socket.socket_path)?;

    match run_command_inner(command, global_options, stream, None)? {
        DaemonResponse::Format(DaemonFormatResponse::Success()) => {
            info!("Success to format");
        }
        DaemonResponse::Format(DaemonFormatResponse::Ignored(reason)) => {
            info!("File ignored. reason: {}", reason);
        }
        DaemonResponse::Format(DaemonFormatResponse::Error(err)) => {
            return Err(anyhow!(err));
        }
        DaemonResponse::PureFormat(DaemonPureFormatResponse::Success(formatted)) => {
            info!("Success to format");
            println!("{}", formatted);
        }
        DaemonResponse::PureFormat(DaemonPureFormatResponse::Ignored(reason)) => {
            info!("File ignored. reason: {}", reason);
        }
        DaemonResponse::PureFormat(DaemonPureFormatResponse::Error(err)) => {
            return Err(anyhow!(err));
        }
        DaemonResponse::BulkFormat(DaemonBulkFormatResponse::Success(message)) => {
            info!("Success to format: {}", message);
        }
        DaemonResponse::BulkFormat(DaemonBulkFormatResponse::Error(err)) => {
            return Err(anyhow!(err));
        }
        DaemonResponse::Stop => {
            info!("Daemon stopped");
        }
        DaemonResponse::Pong(info) => {
            println!("pong!");
            println!("daemon pid: {}", &info.pid);
            println!("daemon start time: {}", &info.start_time);
            println!("daemon log file: {}", &info.stderr_path);
        }
    }

    Ok(())
}
