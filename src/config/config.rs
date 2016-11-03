use rustc_serialize::Decodable;
use std::fs::File;
use std::io::Read;
use toml;

#[derive(Clone, RustcDecodable)]
pub struct FortinbrasConfig {
    pub server_port: String,
    pub ui_port: String,
    pub data_dir: String,
}

pub fn load_config(file_name: String) -> FortinbrasConfig {
    info!("Loading config from {}", file_name);
    let file = read_file(file_name);
    let value = match file.parse::<toml::Value>() {
        Ok(v) => v,
        Err(e) => panic!("Error parsing config file: {:?}", e),
    };
    match FortinbrasConfig::decode(&mut toml::Decoder::new(value)) {
        Ok(x) => x,
        Err(e) => panic!("Error decoding config file: {}", e),
    }
}

fn read_file(file_name: String) -> String {
    let mut f = match File::open(file_name) {
        Ok(file) => file,
        Err(e) => panic!("Error opening config file: {}", e),
    };
    let mut buffer = String::new();
    match f.read_to_string(&mut buffer) {
        Ok(_) => buffer,
        Err(e) => panic!("Error reading config file: {}", e),
    }
}
