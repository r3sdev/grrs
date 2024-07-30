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

fn find_matches(content: BufReader<File>, pattern: &str) {
    for (index, line) in content.lines().enumerate() {
        let line = match line {
            Ok(line) => line,
            Err(_) => {
                eprintln!("Could not read line at index: {}", index);
                continue;
            }
        };

        if line.contains(&pattern) {
            println!("{}", line);
        }
    }
}

fn main() -> Result<()> {
    let args = Cli::parse();
    let path = &args.path;
    let file =
        File::open(path).with_context(|| format!("could not read file `{}`", path.display()))?;

    find_matches(BufReader::new(file), &args.pattern);

    Ok(())
}
