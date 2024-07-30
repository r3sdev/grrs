use std::fs::File;
use std::io::{BufRead, BufReader};

use anyhow::{Context, Result};
use clap::Parser;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    path: std::path::PathBuf,
}

#[derive(Debug)]
#[allow(dead_code)]
struct CustomError(String);

fn main() -> Result<()> {
    let args = Cli::parse();
    let path = &args.path;
    let file =
        File::open(path).with_context(|| format!("could not read file `{}`", path.display()))?;

    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.expect("Unable to read line");

        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }
    Ok(())
}
