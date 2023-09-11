use std::fs;
use std::io;
use std::fs::DirEntry;

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

fn main() {
    let mut entries: Vec<DirEntry> = fs::read_dir(".")
        .expect("Error reading directory")
        .map(|entry| entry.expect("Error getting file name"))
        .collect();

    // sort files by name
    entries.sort_by(|a, b| a.file_name().cmp(&b.file_name()));

    for entry in entries.iter() {
        println!(
            "{:>1} {:>5} {}",
            if is_directory(entry) { "d" } else { "" },
            get_file_size(entry).unwrap_or(0), 
            entry.file_name().to_string_lossy()
        );
    }
}
