use std::path::Path;
use walkdir::{DirEntry, WalkDir};
use anyhow::Result;

fn is_hidden(entry: &DirEntry) -> bool {
    entry.file_name()
         .to_str()
         .map(|s| s.starts_with("."))
         .unwrap_or(false)
}

fn is_file(entry: &DirEntry) -> bool {
    entry.metadata().map(|e| e.is_file()).unwrap_or(false)
}


pub fn for_each_input_file<F: FnMut(&Path) -> Result<()>>(dir: &str, mut clos: F) -> Result<()> {
    WalkDir::new(dir).into_iter()
        .filter_entry(|e| !is_hidden(e)) // Filter out hidden files (.\*)
        .filter_map(|e| e.ok()) // Ignore any errors produced by walkdir
        .filter(|e| is_file(e)) // Skip directories and whatever else is not a file (symbolic links too I guess)
        .for_each(|entry|
            clos(entry.path())
            .unwrap_or_else(|err| println!("{}: an error occurred, namely:\n{}", entry.path().display(), err)));
    Ok(())
}