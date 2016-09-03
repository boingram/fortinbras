extern crate hyper;
extern crate rustc_serialize;

pub mod model;
pub mod server;
pub mod storage;

use server::server::FortinbrasServer;

fn main() {
    FortinbrasServer::launch(String::from("7341"));
}
