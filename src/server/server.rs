use hyper::header::{AccessControlAllowHeaders, AccessControlAllowMethods, AccessControlAllowOrigin};
use hyper::method::Method;
use hyper::server::{Server, Request, Response};
use hyper::status::StatusCode;
use hyper::uri::RequestUri;
use hyper::Url;
use model::item::Item;
use std::io::Read;
use std::sync::RwLock;
use storage::client::StorageClient;
use unicase::UniCase;

/// Web server containing a storage client. Accepts HTTP requests to create, 
/// remove, and update keys.
pub struct FortinbrasServer {
    storage_client: StorageClient,
}

/// Implementation of the Fortinbras Server. Launches server and handles HTTP
/// requests.
impl FortinbrasServer {

    /// Launches a server with a read/write lock wrapped FortinbrasServer that
    /// accesses a storage client. 
    pub fn launch(port: String) {
        let server = RwLock::new(FortinbrasServer { storage_client: StorageClient::new() });

        Server::http(format!("127.0.0.1:{}", port).as_str())
            .unwrap()
            .handle(move |req: Request, mut res: Response| {
                match req.method {
                    Method::Options => server.read().unwrap().options(res),
                    Method::Post => server.write().unwrap().post(req, res),
                    Method::Delete => server.write().unwrap().delete(req, res), 
                    Method::Get => server.read().unwrap().get(req, res),
                    _ => *res.status_mut() = StatusCode::MethodNotAllowed,
                }
            })
            .unwrap();
    }

    /// Respond to preflight requests from fortinbras-ui 
    fn options(&self, mut res: Response) {
        add_cors_headers(&mut res);    
    }

    /// Routes all POST requests to the correct handlers
    fn post(&mut self, req: Request, mut res: Response) {
        add_cors_headers(&mut res);

        let url = match get_url(&req) {
            Some(x) => x,
            None => {
                *res.status_mut() = StatusCode::MethodNotAllowed;
                return;
            }
        };

        match url.path() { 
            "/items" => self.insert_item(req, res),
            _  => {
                *res.status_mut() = StatusCode::NotFound;
            },
        }
    }

    /// Insert a key at POST /items with a body of {"key": "k", "val": "val"}
    fn insert_item(&mut self, mut req: Request, mut res: Response) {
        let mut json = String::new();
        if let Err(_) = req.read_to_string(&mut json) {
            *res.status_mut() = StatusCode::UnprocessableEntity;
        }

        let item = match Item::from_json(&json) {
            Ok(i) => i,
            Err(_) => {
                *res.status_mut() = StatusCode::UnprocessableEntity;
                return;
            }
        };

        match self.storage_client.insert(item.key().clone(), item.val().clone()) {
            _ => {
                *res.status_mut() = StatusCode::Created;
                res.send(json.as_bytes());
                debug!("Server returning {} after write", json);
            }
        }
    }

    /// Routes all DELETE requests to the appropriate handlers.
    fn delete(&mut self, mut req: Request, mut res: Response) {
        add_cors_headers(&mut res);

        let url = match get_url(&req) {
            Some(x) => x,
            None => {
                *res.status_mut() = StatusCode::MethodNotAllowed;
                return;
            }
        };

        match url.path() { 
            "/items" => self.delete_item(req, res, url),
            _  => {
                *res.status_mut() = StatusCode::NotFound;
            },
        }
    }

    /// Deletes a key via DELETE /items?key=k
    fn delete_item(&mut self, mut req: Request, mut res: Response, url: Url) {
        let (query_key, arg) = match url.query_pairs().next() {
            Some((a, b)) => (a.into_owned(), b.into_owned()),
            None => { 
                *res.status_mut() = StatusCode::BadRequest;
                return ;
            }
        };
        if query_key != "key" {
            *res.status_mut() = StatusCode::BadRequest;
            return;
        }

        match self.storage_client.remove(&arg) {
            Some(val) => {
                match Item::new(arg, val.clone()).to_json() {
                    Ok(x) => {
                        res.send(x.as_bytes());
                        debug!("Server returning {} after item deletion", x);
                    }
                    Err(_) => {
                        *res.status_mut() = StatusCode::InternalServerError;
                    }
                };
            }
            None => {
                *res.status_mut() = StatusCode::NotFound;
            }
        }

    }

    /// Routes all GET requests to the appropriate handlers 
    fn get(&self, req: Request, mut res: Response) {
        add_cors_headers(&mut res);

        let url = match get_url(&req) {
            Some(x) => x,
            None => {
                *res.status_mut() = StatusCode::MethodNotAllowed;
                return;
            }
        };

        match url.path() { 
            "/items" => self.get_item(req, res, url),
            _  => {
                *res.status_mut() = StatusCode::NotFound;
            },
        }
    }

    /// Retrieves an item via GET /items?key=k    
    fn get_item(&self, req: Request, mut res: Response, url: Url) { 
        let (query_key, arg) = match url.query_pairs().next() {
            Some((a, b)) => (a.into_owned(), b.into_owned()),
            None => { 
                *res.status_mut() = StatusCode::BadRequest;
                return ;
            }
        };
        if query_key != "key" {
            *res.status_mut() = StatusCode::BadRequest;
            return;
        }

        let cli_res = self.storage_client.get(&arg);
        match cli_res {
            Some(val) => {
                match Item::new(arg, val.clone()).to_json() {
                    Ok(x) => {
                        res.send(x.as_bytes());
                        debug!("Server returning {} on read", x);
                    }
                    Err(_) => {
                        *res.status_mut() = StatusCode::InternalServerError;
                    }
                };
            }
            None => *res.status_mut() = StatusCode::NotFound,
        }
    }
}

/// Given a request, parse out the url 
fn get_url<>(req: &Request) -> Option<Url> {
    let path = match req.uri {
        RequestUri::AbsolutePath(ref path) => path,
        _ => {
            return None        
        }
    };

    let base_url = Url::parse(&"http://localhost:7341").unwrap();
    Some(base_url.join(&path).unwrap())
}

/// Add CORS headers to an outgoing response
fn add_cors_headers(res: &mut Response) {
    res.headers_mut().set(
        AccessControlAllowHeaders(vec![UniCase("Content-Type".to_owned())])
    );
    res.headers_mut().set(
        AccessControlAllowMethods(vec![Method::Get, Method::Post, Method::Delete])
    );
    res.headers_mut().set(
        AccessControlAllowOrigin::Value("*".to_owned())
    );
}
