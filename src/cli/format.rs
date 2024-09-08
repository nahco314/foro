use crate::app_dir::cache_dir_res;
use crate::cli::GlobalOptions;
use crate::config;
use crate::config::{get_or_create_default_config, load_config_and_cache, load_file, Command};
use crate::handle_plugin::run::run;
use anyhow::{Context, Error, Result};
use clap::builder::{IntoResettable, Resettable, ValueHint};
use clap::Parser;
use log::{debug, info};
use nix::unistd::{fork, ForkResult};
use os_pipe::PipeWriter;
use serde_json::json;
use std::env::current_dir;
use std::fmt::Display;
use std::io::{stdout, Read, Write};
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::mpsc;
use std::thread::sleep;
use std::time::{Instant, SystemTime, UNIX_EPOCH};
use std::{fs, io, process, thread, time};
use url::Url;
use url_serde::{Serde, SerdeUrl};

#[derive(Parser, Debug)]
pub struct FormatArgs {
    /// Path to format
    pub path: PathBuf,
}

pub fn format_execute_with_args(args: FormatArgs, global_options: GlobalOptions) -> Result<()> {
    let no_quick_magic =
        std::env::var_os("ONEFMT_NO_QUICK_MAGIC").is_some_and(|s| s != "0" && s != "");

    debug!("no_quick_magic: {}", no_quick_magic);

    let mut maybe_write: Option<PipeWriter> = None;

    if !no_quick_magic {
        let target_path = args.path.canonicalize()?;

        let (mut reader, mut writer) = os_pipe::pipe()?;

        info!("open pipe");

        match unsafe { fork()? } {
            ForkResult::Parent { child } => {
                let metadata = fs::metadata(&target_path)?;
                let modified_time = metadata.modified()?;

                loop {
                    let metadata = fs::metadata(&target_path)?;
                    let new_modified_time = metadata.modified()?;

                    if new_modified_time != modified_time {
                        info!("quick magic detected file changed");
                        break;
                    }

                    writer.write_all(b"0")?;
                    let mut output = [0];
                    reader.read_exact(&mut output)?;

                    if output[0] == b'1' {
                        info!("quick magic detected child finished");
                        break;
                    }

                    sleep(time::Duration::from_micros(100));
                }

                info!("main process exit");

                let now = SystemTime::now();

                // UNIXエポックからの経過時間を取得
                let since_the_epoch = now.duration_since(UNIX_EPOCH).expect("Time went backwards");

                // 秒とナノ秒をそれぞれ取得
                let seconds = since_the_epoch.as_secs();
                let nanoseconds = since_the_epoch.subsec_nanos();

                // マイクロ秒単位の精度を計算
                let microseconds = nanoseconds / 1_000;
                println!("{}.{:06}", seconds, microseconds);

                process::exit(0);
            }
            ForkResult::Child => {
                maybe_write = Some(writer);
            }
        }
    }

    let (config, cache_dir) =
        load_config_and_cache(&global_options.config_file, &global_options.cache_dir)?;

    let file = fs::File::open(&args.path)?;
    let mut buf_reader = io::BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;

    let res = run(
        &config.rules.first().unwrap().cmd,
        json!({
            "current-dir": current_dir()?.canonicalize()?.to_str().unwrap(),
            "target": args.path.canonicalize()?.to_str().unwrap(),
            "raw-target": args.path,
            "target-content": contents,
            }
        ),
        &cache_dir,
        !global_options.no_cache,
    )?;

    if let Some(mut w) = maybe_write {
        // err is maybe not important (broken pipe)
        let _err = w.write_all(b"1");
    } else {
        println!("{:?}", res);
    }

    Ok(())
}
