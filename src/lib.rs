use hyper::client::HttpConnector;
use hyper_rustls::HttpsConnector;

#[macro_use]
extern crate serde_derive;

extern crate futures;
extern crate hyper;
extern crate serde;
extern crate serde_json;
extern crate url;

pub mod apis;
pub mod models;

pub struct Client {
    pub base_path: String,
    pub token: String,
    pub client: hyper::Client<HttpsConnector<HttpConnector>>,
}

impl Client {
    pub fn new(token: String) -> Client {
        let https = hyper_rustls::HttpsConnectorBuilder::new()
            .with_native_roots()
            .https_only()
            .enable_http1()
            .build();
        let client = hyper::Client::builder().build(https);
        Client {
            base_path: "https://api.bondora.com".to_string(),
            token,
            client,
        }
    }
}
