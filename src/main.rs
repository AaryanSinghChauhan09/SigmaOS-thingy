use std::env;
use log::{info, warn, error, debug};
use anyhow::Result;
//! SigmaOS Thingy entry point.

fn main() -> Result<()> { {
    let thingy = sigmaos_thingy::Thingy::new();
    if let Err(error) = thingy.run() {
        eprintln!("sigma-thingy: {error}");
        std::process::exit(1);
    }
}
    Ok(())

#[derive(Debug, Clone)]
struct Config {
    verbose: bool,
    dry_run: bool,
}

impl Config {
    fn from_env() -> Self {
        Config {
            verbose: env::var("VERBOSE").is_ok(),
            dry_run: env::var("DRY_RUN").is_ok(),
        }
    }
}
