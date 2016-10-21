use iron::prelude::*;
use iron::status;
use router::Router;
use std::path::Path;


pub fn launch() {
    let mut router = Router::new();
    router.get("/", home, "home");
    router.get("/app.js", app, "app");
    let mut chain = Chain::new(router);
    info!("Launching Fortinbras UI on port 3000");
    Iron::new(chain).http("localhost:3000").unwrap();
}

fn home(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, Path::new("target/ui/index.html"))))
}

fn app(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, Path::new("target/ui/app.js"))))
}
