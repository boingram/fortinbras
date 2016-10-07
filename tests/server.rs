extern crate fortinbras;
extern crate hyper;

use fortinbras::model::item::Item;
use fortinbras::server::server::FortinbrasServer;
use hyper::Client;
use hyper::status::StatusCode;
use std::io::Read;
use std::thread;

fn launch_fortinbras_server() {
    thread::spawn( || {
        FortinbrasServer::launch(String::from("7341"));
    });
}

#[test]
fn test_write_key_retrieve_it_delete_it() {
    launch_fortinbras_server();
    write_item(String::from("a"), String::from("b"));
    get_item(String::from("a"), String::from("b"), StatusCode::Ok);
    get_item(String::from("A"), String::from("b"), StatusCode::Ok);
    delete_item(String::from("a"), String::from("b"));
    get_item(String::from("a"), String::from("doesnt matter"), StatusCode::NotFound);
}

fn write_item(key: String, val: String) {
    let client = Client::new();

    let res = client.post("http://localhost:7341/items")
                    .body(&Item::new(key, val).to_json().unwrap())
                    .send()
                    .unwrap();
    
    assert_eq!(res.status, StatusCode::Created);
}

fn get_item(key: String, expected_val: String, expected_response: StatusCode) {
    let client = Client::new();
    let uri = format!("http://localhost:7341/items?key={}", key);
    let mut res = client.get(&uri).send().unwrap();

    assert_eq!(res.status, expected_response);

    if expected_response == StatusCode::Ok {
        let mut body = String::new();
       
        let size = match res.read_to_string(&mut body) {
            Ok(x) => x,
            _     => 0,
        };

        assert!(size > 0);

        let item = Item::from_json(&body).unwrap();
        assert_eq!(item.key(), &key);
        assert_eq!(item.val(), &expected_val);
    }
}

fn delete_item(key: String, expected_val: String) {
    let client = Client::new();
    let uri = format!("http://localhost:7341/items?key={}", key);
    let mut res = client.delete(&uri).send().unwrap();

    assert_eq!(res.status, StatusCode::Ok);
    
    let mut body = String::new();
    let size = match res.read_to_string(&mut body) {
        Ok(x) => x,
        _     => 0,
    };

    assert!(size > 0);

    let item = Item::from_json(&body).unwrap();
    assert_eq!(item.key(), &key);
    assert_eq!(item.val(), &expected_val);
}
