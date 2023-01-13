#![forbid(unsafe_code)]
#![deny(warnings, missing_docs)]

//! Binary to test if semantic-release is configured.
//!
//! The intended use-case is in shell-script control flow.
//!
//! Note: this tool currently only checks for the existence of configuration,
//! it does not validate the content. This tool does not care if your
//! configuration will be rejected by semantic-release.

use std::error::Error;
use std::path::PathBuf;

use clap::Parser;

use find_semantic_release_config::find_semantic_release_configuration;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Cli {
    /// Path to project root
    #[arg(long, default_value = ".")]
    root: PathBuf,
}

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("warn")).init();

    let cli = Cli::parse();

    let is_configured = find_semantic_release_configuration(&cli.root)?;

    if is_configured.is_some() {
        return Ok(());
    } else {
        std::process::exit(1);
    }
}
