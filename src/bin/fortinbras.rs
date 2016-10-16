extern crate fortinbras;

#[macro_use]
extern crate log;

extern crate log4rs;

use fortinbras::server::server::FortinbrasServer;

fn main() {
    log4rs::init_file("config/log.yaml", Default::default()).unwrap();

    info!("Launching Fortinbras...");

    FortinbrasServer::launch(String::from("7341"));
}
