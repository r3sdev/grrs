use std::fs::File;
use std::io::{BufReader, Read};
// Used for writing assertions
use std::process::Command;

use anyhow::{Context, Result};
use assert_cmd::prelude::*;
use assert_fs::prelude::*;
use clap::Parser;
use predicates::prelude::*;

// Add methods on commands
use grrs::find_matches;

// Run programs
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn file_doesnt_exist() -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = Command::cargo_bin("grrs")?;

        cmd.arg("foobar").arg("test/file/doesnt/exist");
        cmd.assert()
            .failure()
            .stderr(predicate::str::contains("could not read file"));

        Ok(())
    }

    #[test]
    fn find_content_in_file() -> Result<(), Box<dyn std::error::Error>> {
        let file = assert_fs::NamedTempFile::new("sample.txt")?;
        file.write_str("A test\nActual content\nMore content\nAnother test")?;

        let mut cmd = Command::cargo_bin("grrs")?;
        cmd.arg("test").arg(file.path());
        cmd.assert()
            .success()
            .stdout(predicate::str::contains("A test\nAnother test"));

        Ok(())
    }
}
