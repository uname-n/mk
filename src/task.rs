use crate::config::TaskSettings;
use crate::error::RError;
use crate::opts::Opts;
use crate::util::{
    create_log_directory, current_timestamp, log_file, print_separator, replace_env_vars,
};

use anyhow::{Context, Result};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::process::Stdio;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use termion::color;
use tokio::process::{Child, Command as TokioCommand};
use tokio::signal;

pub async fn run_task(
    command: &[String],
    background: bool,
    index: usize,
    timestamp: &str,
) -> Result<Child> {
    let log_dir = PathBuf::from(".log/mk").join(timestamp);
    if background {
        println!(
            "{}mk:: [{}{:?}{}] background task:{} {:?}{}",
            color::Fg(color::LightBlack),
            color::Fg(color::LightGreen),
            index,
            color::Fg(color::LightBlack),
            color::Fg(color::LightGreen),
            command,
            color::Fg(color::Reset)
        );

        create_log_directory(&log_dir)?;
    }

    let process = command
        .first()
        .ok_or(RError::TaskStartError(command.to_vec()))?;
    let args = &command[1..];
    let output = if background {
        log_file(&log_dir, index)?
    } else {
        Stdio::inherit()
    };
    let stderr = if background {
        Stdio::null()
    } else {
        Stdio::inherit()
    };

    let child = TokioCommand::new(process)
        .args(args)
        .stdout(output)
        .stderr(stderr)
        .spawn()
        .with_context(|| {
            format!(
                "{}mk:: {}failed to spawn command: {:?}{}",
                color::Fg(color::LightBlack),
                color::Fg(color::Red),
                command[0],
                color::Fg(color::Reset)
            )
        })?;

    Ok(child)
}

pub async fn run_background_tasks(tasks: &[Vec<String>], timestamp: &str) -> Result<Vec<Child>> {
    let mut children = Vec::with_capacity(tasks.len());
    for (index, task) in tasks.iter().enumerate() {
        let task_with_env = replace_env_vars(task)?;
        children.push(run_task(&task_with_env, true, index + 1, timestamp).await?);
    }
    Ok(children)
}

pub async fn handle_command(opts: Opts, shutdown_signal: Arc<AtomicBool>) -> Result<()> {
    let settings = read_settings(&opts.config).await?;
    let task = settings.get(&opts.command).ok_or(RError::TaskNotFound)?;
    let timestamp = current_timestamp();

    println!(
        "{}{}{}",
        termion::clear::All,
        termion::cursor::Goto(1, 1),
        termion::cursor::Hide,
    );
    println!(
        "{}mk:: {}running {}{:?}{}",
        color::Fg(color::LightBlack),
        color::Fg(color::Green),
        color::Fg(color::LightGreen),
        opts.command,
        color::Fg(color::Reset)
    );

    let background_children = if let Some(bg_tasks) = &task.background_tasks {
        run_background_tasks(bg_tasks, &timestamp).await?
    } else {
        vec![]
    };
    let preprocessed_main_command = replace_env_vars(&task.command)?;

    let mut main_command_child = run_task(&preprocessed_main_command, false, 0, &timestamp).await?;

    print_separator();
    print!("\r\n");

    loop {
        tokio::select! {
            _ = main_command_child.wait() => {
                break;
            },
            _ = signal::ctrl_c() => {
                shutdown_signal.store(true, Ordering::SeqCst);
                main_command_child.kill().await?;
                break;
            },
        }
    }

    print!("\r\n");

    if shutdown_signal.load(Ordering::SeqCst) {
        print_separator();
        println!(
            "{}mk:: {}ctrl-c signal recieved. shutting down...{}",
            color::Fg(color::LightBlack),
            color::Fg(color::Green),
            color::Fg(color::Reset)
        );
    }

    if background_children.len() > 0 {
        print_separator();
        for mut child in background_children {
            println!(
                "{}mk:: exiting background task. pid={}{:?}{}",
                color::Fg(color::LightBlack),
                color::Fg(color::LightGreen),
                child.id().unwrap(),
                color::Fg(color::Reset)
            );
            if let Err(e) = child.kill().await {
                eprintln!(
                    "{}mk:: {}error killing background task: {:?}{}",
                    color::Fg(color::LightBlack),
                    color::Fg(color::Red),
                    e,
                    color::Fg(color::Reset)
                );
            }
            if let Err(e) = child.wait().await {
                eprintln!(
                    "{}mk:: {}error waiting for background task to terminate: {:?}{}",
                    color::Fg(color::LightBlack),
                    color::Fg(color::Red),
                    e,
                    color::Fg(color::Reset)
                );
            }
        }
    }

    print_separator();
    println!(
        "{}mk:: {}done{} ",
        color::Fg(color::LightBlack),
        color::Fg(color::Green),
        color::Fg(color::Reset)
    );
    println!("{}", termion::cursor::Show);

    Ok(())
}

pub async fn read_settings(config_path: &PathBuf) -> Result<HashMap<String, TaskSettings>> {
    let contents = fs::read_to_string(config_path)?;
    let settings: HashMap<String, TaskSettings> = toml::from_str(&contents)?;
    Ok(settings)
}
