use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;
use superdupergrep::grep_file;
use walkdir::WalkDir;

#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file or directory to search
    path: PathBuf,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    if args.path.is_dir() {
        // recursively search for files in the directory
        for entry in WalkDir::new(&args.path) {
            let entry = entry?;
            if entry.file_type().is_file() {
                grep_file(&entry.path(), &args.pattern, &mut std::io::stdout())?;
            }
        }
    } else {
        grep_file(&args.path, &args.pattern, &mut std::io::stdout())?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use assert_cmd::Command;
    use assert_fs::prelude::FileWriteStr;

    #[test]
    pub fn find_content_in_file() -> Result<(), Box<dyn std::error::Error>> {
        let file = assert_fs::NamedTempFile::new("sample.txt")?;
        file.write_str("Hi!\nHello!\nWhat's up!\n")?;

        let mut cmd = Command::cargo_bin("superdupergrep")?;
        cmd.arg("hello").arg(file.path());
        cmd.assert()
            .success()
            .stdout(predicates::str::contains("Hello!\n"));
        Ok(())
    }
}
