use anyhow::Result;
use clap::Parser;
use colored::Colorize;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

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
        for entry in WalkDir::new(&args.path) {
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
    let content = match fs::read_to_string(path) {
        Ok(content) => content,
        Err(_) => return Ok(()), // skip if it's not UTF-8
    };
    for (line_number, line) in content.lines().enumerate() {
        if line
            .to_lowercase()
            .contains(&pattern.to_string_lossy().to_lowercase())
        {
            println!("{}", path.display().to_string().green());
            println!(
                "{}:{}",
                (line_number + 1).to_string().cyan(),
                line.trim().white(),
            );
        }
    }
    Ok(())
}
