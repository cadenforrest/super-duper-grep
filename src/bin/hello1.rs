use anyhow::{Context, Result};
use clap::Parser;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir;

#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    pattern: PathBuf,
    /// The path to the file or directory to search
    path: PathBuf,
}

fn main() -> Result<()> {
    let args = Cli::parse();

    if args.path.is_dir() {
        // recursively search for files in the directory
        for entry in walkdir::WalkDir::new(&args.path) {
            let entry = entry?;
            if entry.file_type().is_file() {
                grep_file(&entry.path(), &args.pattern)?;
            }
        }
    } else {
        // search the single file
        grep_file(&args.path, &args.pattern)?;
    }

    Ok(())
}

// Matches content in a file, prints to std::out
fn grep_file(path: &Path, pattern: &Path) -> Result<()> {
    let content =
        fs::read_to_string(path).with_context(|| format!("Error reading `{}`", path.display()))?;

    for (line_number, line) in content.lines().enumerate() {
        if line
            .to_lowercase()
            .contains(&pattern.to_string_lossy().to_lowercase())
        {
            println!("[{}]:{} - {}", path.display(), line_number + 1, line.trim());
        }
    }

    Ok(())
}
