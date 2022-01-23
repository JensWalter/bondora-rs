use hyper;
use serde;
use serde_json;
use hyper::Client;
use hyper_rustls::HttpsConnector;
use hyper::client::HttpConnector;

#[derive(Debug)]
pub enum Error<T> {
    Hyper(hyper::Error),
    Serde(serde_json::Error),
    ApiError(ApiError<T>),
}

#[derive(Debug)]
pub struct ApiError<T> {
    pub code: hyper::StatusCode,
    pub content: Option<T>,
}

impl<'de, T> From<(hyper::StatusCode, &'de [u8])> for Error<T> 
    where T: serde::Deserialize<'de> {
    fn from(e: (hyper::StatusCode, &'de [u8])) -> Self {
        if e.1.len() == 0 {
            return Error::ApiError(ApiError{
                code: e.0,
                content: None,
            });
        }
        match serde_json::from_slice::<T>(e.1) {
            Ok(t) => Error::ApiError(ApiError{
                code: e.0,
                content: Some(t),
            }),
            Err(e) => {
                Error::from(e)
            }
        }
    }
}

impl<T> From<hyper::Error> for Error<T> {
    fn from(e: hyper::Error) -> Self {
        return Error::Hyper(e)
    }
}

impl<T> From<serde_json::Error> for Error<T> {
    fn from(e: serde_json::Error) -> Self {
        return Error::Serde(e)
    }
}

mod account_api;
pub use self::account_api::{ AccountApi };
mod auction_api;
pub use self::auction_api::{ AuctionApi };
mod bid_api;
pub use self::bid_api::{ BidApi };
mod report_api;
pub use self::report_api::{ ReportApi };
mod second_market_api;
pub use self::second_market_api::{ SecondMarketApi };

pub struct APIClient {
    pub base_path: String,
    pub token: String,
    pub client: Client<HttpsConnector<HttpConnector>>,
  }
  
  impl APIClient {
    pub fn new(token: String) -> APIClient {
      let https = hyper_rustls::HttpsConnectorBuilder::new()
              .with_native_roots()
              .https_only()
              .enable_http1()
              .build();
      let client = hyper::Client::builder().build(https);
      APIClient {
        base_path: "https://api.bondora.com".to_string(),
        token,
        client,
      }
    }
  }
  