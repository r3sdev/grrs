use std::fs::File;
use std::io::{BufReader, Read};

use anyhow::{Context, Result};
use clap::Parser;

use grrs::find_matches;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    path: std::path::PathBuf,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    let path = &args.path;
    let mut file = BufReader::new(
        File::open(path).with_context(|| format!("could not read file `{}`", path.display()))?,
    );
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();

    find_matches(&content, &args.pattern, &mut std::io::stdout());

    Ok(())
}
