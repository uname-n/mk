use anyhow::{Context, Result};
use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Stdio;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use std::time::{SystemTime, UNIX_EPOCH};
use termion::color;
use tokio::signal;

pub fn create_log_directory(log_dir: &PathBuf) -> Result<()> {
    fs::create_dir_all(log_dir).with_context(|| {
        format!(
            "{}mk:: {}failed to create log directory: {:?}{}",
            color::Fg(color::LightBlack),
            color::Fg(color::Red),
            log_dir,
            color::Fg(color::Reset)
        )
    })
}

pub fn log_file(log_dir: &PathBuf, index: usize) -> Result<Stdio> {
    let log_path = log_dir.join(format!("background-task.{}.log", index));
    let file = fs::File::create(log_path)?;
    Ok(Stdio::from(file))
}

pub fn replace_env_vars(command: &[String]) -> Result<Vec<String>> {
    dotenv::dotenv().ok();
    command
        .iter()
        .map(|arg| {
            if arg.starts_with('$') {
                let key = arg.trim_start_matches('$');
                env::var(key).map_err(|_| {
                    anyhow::Error::msg(format!(
                        "{}mk:: {}failed to read environment variable: {}{}",
                        color::Fg(color::LightBlack),
                        color::Fg(color::Red),
                        key,
                        color::Fg(color::Reset)
                    ))
                })
            } else {
                Ok(arg.clone())
            }
        })
        .collect()
}

pub async fn ctrl_c_listener(shutdown_signal: Arc<AtomicBool>) {
    let _ = signal::ctrl_c().await;
    shutdown_signal.store(true, Ordering::SeqCst);
}

pub fn current_timestamp() -> String {
    let since_epoch = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect(&format!(
            "{}mk:: {}time went backwards{}",
            color::Fg(color::LightBlack),
            color::Fg(color::Red),
            color::Fg(color::Reset)
        ));
    since_epoch.as_secs().to_string()
}

pub fn print_separator() {
    println!(
        "{}mk:: {}{}",
        color::Fg(color::LightBlack),
        "= ".repeat(20) + "=",
        color::Fg(color::Reset)
    );
}
