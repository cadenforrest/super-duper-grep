use anyhow::Result;
use colored::Colorize;
use std::fs;
use std::path::Path;

// TODO: this probably shouldn't be concerned with printing things idk
/// Matches content in a file, writes results to writer
pub fn grep_file(path: &Path, pattern: &String, mut writer: impl std::io::Write) -> Result<()> {
    let content = match fs::read_to_string(path) {
        Ok(content) => content,
        Err(_) => return Ok(()), // skip if it's not UTF-8
    };
    for (line_number, line) in content.lines().enumerate() {
        if line.to_lowercase().contains(&pattern.to_lowercase()) {
            writeln!(writer, "{}", path.display().to_string().green())?;
            writeln!(
                writer,
                "{}:{}",
                (line_number + 1).to_string().cyan(),
                line.trim().white(),
            )?;
        }
    }
    Ok(())
}
