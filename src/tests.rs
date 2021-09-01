use rocket::local::blocking::Client;
use rocket::http::Status;

#[test]
fn hello_world_not_found() {
    let client = Client::tracked(super::rocket()).unwrap();
    let response = client.get("/hello/world").dispatch();
    assert_eq!(response.status(), Status::NotFound);
}


#[test]
fn hello_world_success() {
    let client = Client::tracked(super::rocket()).unwrap();
    let response = client.get("/").dispatch();
    assert_eq!(response.status(), Status::Ok);
}
