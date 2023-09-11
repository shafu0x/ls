use std::fs;
use std::io;
use std::fs::DirEntry;
use std::os::unix::fs::PermissionsExt;

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
    result.push(if mode & 0o400 != 0 { 'r' } else { '-' });
    result.push(if mode & 0o200 != 0 { 'w' } else { '-' });
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
        println!(
            "{:<1} {:<5} {:<8} {}",
            if is_directory(entry) { "d" } else { "" },
            parse_permissions(entry).unwrap_or(String::from("---------")),
            get_file_size(entry).unwrap_or(0), 
            entry.file_name().to_string_lossy()
        );
    }
}
