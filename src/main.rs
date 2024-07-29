use std::fs::File;
use std::io::{BufRead, BufReader};

use clap::Parser;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::parse();
    let path = &args.path;

    let f = File::open(path).expect("Unable to open file");
    let reader = BufReader::new(f);

    for line in reader.lines() {
        let line = line.expect("Unable to read line");

        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }
}
