mod config;
mod error;
mod opts;
mod task;
mod util;

use crate::opts::Opts;
use anyhow::Result;
use clap::Parser;
use std::sync::{atomic::AtomicBool, Arc};

#[tokio::main]
async fn main() -> Result<()> {
    let opts = Opts::parse();
    let shutdown_signal = Arc::new(AtomicBool::new(false));
    tokio::spawn(util::ctrl_c_listener(shutdown_signal.clone()));
    task::handle_command(opts, shutdown_signal.clone()).await?;
    Ok(())
}
