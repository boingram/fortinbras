extern crate fortinbras;

#[macro_use]
extern crate log;

extern crate log4rs;

use fortinbras::server::server::FortinbrasServer;
use fortinbras::server::ui;
use std::thread;

fn main() {
    log4rs::init_file("config/log.yaml", Default::default()).unwrap();

    info!("Launching Fortinbras...");

    thread::spawn(|| ui::launch());

    FortinbrasServer::launch(String::from("7341"));
}
