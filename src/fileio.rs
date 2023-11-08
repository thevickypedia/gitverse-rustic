use std::fs::{File, remove_file};
use std::io::{Error, Write};
use std::path::Path;
use std::process::exit;

use log::{info, warn};

fn write_file(filename: &str, title: &str, snippets: Vec<String>) -> Result<usize, Error> {
    let mut file = File::create(&filename)?;
    // write_all is safer but it returns an empty tuple () instead of usize
    let init = format!("{}\n{}\n\n", title, "=".repeat(title.len()));
    let mut bytes = file.write(init.as_bytes())?;
    for (index, each_snippet) in snippets.iter().enumerate() {
        let snip;
        if index + 1 < snippets.len() {
            snip = format!("{}\n", each_snippet)
        } else {
            snip = each_snippet.to_string();
        }
        match file.write(snip.as_bytes()) {
            Ok(bytes_written) => {
                bytes += bytes_written;
            }
            Err(error) => {
                println!("Failed to write data into {}: {}", filename, error);
                exit(1)
            }
        }
    }
    Ok(bytes)
}


pub fn dump_release_notes(filename: &str, title: &str, snippets: Vec<String>) {
    let file = Path::new(filename);
    if file.exists() {
        warn!("Found existing '{}'. Recreating now.", filename);
        match remove_file(file) {
            Ok(_) => {}
            Err(error) => {
                // should be printed regardless of debug state
                println!("Error deleting the existing file: {}", error);
                exit(1)
            }
        }
    }
    match write_file(filename, title, snippets) {
        Ok(bytes) => {
            info!("Bytes written: {}", bytes)
        }
        Err(error) => {
            println!("Failed to write data into '{}': {}", filename, error);
            exit(1)
        }
    }
}
