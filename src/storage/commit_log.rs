use std::fs;
use std::path::Path;



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
    let path = Path::new(name);
    if !path.exists() {
        create_file(path);
    }
    match OpenOptions::new().append(true).open(name) {
        Ok(f) => f,
        Err(e) => {
            panic!("Error opening commit log file {}: {}", name, e);
        }
    }
}
