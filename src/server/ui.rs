use iron::prelude::*;
use iron::status;
use router::Router;
use std::path::Path;

/// Launch Fortinbras UI, a separately compiled Elm app, and serve it up on port 3000
pub fn launch() {
    let mut router = Router::new();
    router.get("/", home, "home");
    router.get("/app.js", app, "app");
    let chain = Chain::new(router);
    info!("Launching Fortinbras UI on port 3000");
    Iron::new(chain).http("localhost:3000").unwrap();
}

/// Route requests to / to get the html
fn home(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, Path::new("target/ui/index.html"))))
}

/// Route requests to retrieve the javascript bundle holding fortinbras ui
fn app(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, Path::new("target/ui/app.js"))))
}
