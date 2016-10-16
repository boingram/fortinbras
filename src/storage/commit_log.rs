use model::item::Item;
use std::fs;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Error;
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
        let json = format!("{}\n", item.to_json().unwrap());
        self.file.write(json.as_bytes())
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
    match OpenOptions::new().create(true).append(true).open(name) {
        Ok(f) => f,
        Err(e) => {
            panic!("Error opening commit log file {}: {}", name, e);
        }
    }
}
