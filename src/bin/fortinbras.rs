extern crate fortinbras;

#[macro_use]
extern crate log;

extern crate log4rs;

use fortinbras::server::server::FortinbrasServer;
use fortinbras::server::ui;
use fortinbras::config::config;
use std::thread;

fn main() {
    log4rs::init_file("config/log.yaml", Default::default()).unwrap();

    let cfg = config::load_config(String::from("config/settings.toml"));

    info!("Launching Fortinbras...");

    let ui_cfg = cfg.clone();
    thread::spawn(move || ui::launch(ui_cfg.ui_port));

    let server_cfg = cfg.clone();
    FortinbrasServer::launch(server_cfg.server_port, server_cfg.data_dir);
}
