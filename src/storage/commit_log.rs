use bincode::rustc_serialize::{encode, decode};
use bincode::SizeLimit;
use model::item::Item;
use std::fs;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::BufReader;
use std::io::Error;
use std::io::Read;
use std::io::Write;
use std::path::Path;

/// The commit log is an interface for a literal file on disk that
/// contains the current state of the in memory map. The commit log
/// exists so that when fortinbras is shut down ungracefully,
/// we have a way to recover the data in memory. Any write to the
/// in memory map first is written to the commit log.
pub struct CommitLog {
    file: File,
}

impl CommitLog {
    /// Initialize the commit log, creating the data dir and file if
    /// necessary.
    pub fn init(data_dir: &str) -> CommitLog {
        check_dir(data_dir);
        let file = get_file(&format!("{}/commit.log", data_dir));
        CommitLog { file: file }
    }

    /// Write a given item to the commit log.
    pub fn write(&mut self, item: &Item) -> Result<usize, Error> {
        let encoded: Vec<u8> = encode(item, SizeLimit::Infinite).unwrap();
        info!("Writing {} bytes to commit log", encoded.len());
        self.file.write(&encoded)
    }

    /// Read all items from commit log
    pub fn read_items(&self) -> Vec<Item> {
        let mut items = Vec::new();
        let mut file = BufReader::new(&self.file);
        let mut encoded: Vec<u8> = Vec::new();
        let log = file.read_to_end(&mut encoded);

        match log {
            Ok(_) => {
                while encoded.len() > 0 {
                    match decode(&encoded) {
                        Ok(item) => {
                            encoded = encoded.split_off(get_encoded_size(&item));
                            items.push(item);
                        }
                        Err(e) => error!("Error deserializing commit log: {}", e),
                    }
                }
            }
            Err(e) => error!("Error reading from commit log: {}", e),
        }

        info!("{} items read from commit log", items.len());
        items
    }
}

/// Check if a directory exists, create it if it doesn't
fn check_dir(dir: &str) {
    let path = Path::new(dir);
    match fs::read_dir(path) {
        Ok(_) => return,
        Err(_) => create_dir(&path),
    }
}

/// Create a directory at the specified path
fn create_dir(dir: &Path) {
    match fs::create_dir(dir) {
        Ok(_) => info!("Created data storage directory: {}", dir.to_str().unwrap()),
        Err(e) => {
            panic!("Error creating directory {}: {}", dir.to_str().unwrap(), e);
        }
    }
}

/// Get the commit log file
fn get_file(name: &str) -> File {
    match OpenOptions::new()
        .create(true)
        .read(true)
        .append(true)
        .open(name) {
        Ok(f) => f,
        Err(e) => {
            panic!("Error opening commit log file {}: {}", name, e);
        }
    }
}

/// Get the size of an object when encoded
fn get_encoded_size(item: &Item) -> usize {
    let encoded: Vec<u8> = encode(item, SizeLimit::Infinite).unwrap();
    encoded.len()
}
