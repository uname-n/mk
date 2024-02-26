use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[clap(about, version, author)]
pub struct Opts {
    pub command: String,
    #[clap(short = 'c', long, default_value = "mk.toml")]
    pub config: PathBuf,
}
