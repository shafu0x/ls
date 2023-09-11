use std::fs;
use std::fs::DirEntry;
use std::io;
use std::os::unix::fs::PermissionsExt;

extern crate colored;
use colored::*;

fn is_directory(entry: &DirEntry) -> bool {
    if let Ok(file_type) = entry.file_type() {
        file_type.is_dir()
    } else {
        false
    }
}

fn get_file_size(entry: &DirEntry) -> io::Result<u64> {
    let metadata = entry.metadata()?;
    Ok(metadata.len())
}

fn parse_permissions(entry: &DirEntry) -> io::Result<String> {
    let metadata = entry.metadata()?;
    let permissions = metadata.permissions();
    let mode = permissions.mode();
    let mut result = String::new();
    result.push(if mode & 0o100 != 0 { 'x' } else { '-' });
    Ok(result)
}

fn main() {
    let mut entries: Vec<DirEntry> = fs::read_dir(".")
        .expect("Error reading directory")
        .map(|entry| entry.expect("Error getting file name"))
        .collect();

    // sort files by name
    entries.sort_by(|a, b| a.file_name().cmp(&b.file_name()));

    for entry in entries.iter() {
        let is_dir = is_directory(entry);
        println!(
            "{}{:<2} {:<6} {}",
            if is_dir { "d" } else { "-" },
            parse_permissions(entry).unwrap_or(String::from("---------")),
            get_file_size(entry).unwrap_or(0),
            entry.file_name().to_string_lossy().color(if is_dir {
                Color::Blue
            } else {
                Color::White
            })
        );
    }
}
