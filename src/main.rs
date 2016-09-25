extern crate hyper;

#[macro_use]
extern crate log;

extern crate log4rs;

extern crate rustc_serialize;

extern crate unicase;

pub mod model;
pub mod server;
pub mod storage;

use server::server::FortinbrasServer;

fn main() {
    log4rs::init_file("../config/log.yaml", Default::default()).unwrap();
   
    info!("Launching Fortinbras...");

    FortinbrasServer::launch(String::from("7341"));
}
