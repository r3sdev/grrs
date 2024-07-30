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

    let result = File::open(path);
    let file = match result {
        Ok(file) => file,
        Err(error) => {
            panic!("Can't deal with {}, just exit here", error);
        }
    };

    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.expect("Unable to read line");

        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }
}
