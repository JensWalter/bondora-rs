use crate::models::ApiResult;
use crate::models::ApiResultBid;
use crate::models::ApiResultBids;
use crate::models::ApiResultMakeBids;
use crate::Client;
use async_trait::async_trait;
use hyper;
use hyper::{header, Method};
use serde_json;

#[async_trait]
pub trait BidApi {
    async fn bid_cancel_bid(&self, id: &str) -> Result<ApiResult, String>;
    async fn bid_get_bid(&self, id: &str) -> Result<ApiResultBid, String>;
    async fn bid_get_bid_summaries(
        &self,
        request_bid_status_code: i32,
        request_start_date: String,
        request_end_date: String,
        request_page_size: i32,
        request_page_nr: i32,
    ) -> Result<ApiResultBids, String>;
    async fn bid_make_bids(
        &self,
        bid_request: crate::models::BidRequest,
    ) -> Result<ApiResultMakeBids, String>;
}

#[async_trait]
impl BidApi for Client {
    async fn bid_cancel_bid(&self, id: &str) -> Result<ApiResult, String> {
        let token = &self.token;
        let mut builder = hyper::Request::builder()
            .method(Method::POST)
            .header(header::AUTHORIZATION, format!("Bearer {token}"));

        let uri_str = format!("{}/api/v1/bid/{id}/cancel", self.base_path);
        let uri: hyper::Uri = uri_str.parse().unwrap();

        builder = builder.uri(uri);
        let req = builder.body(hyper::Body::empty()).unwrap();

        let https = hyper_rustls::HttpsConnectorBuilder::new()
            .with_native_roots()
            .https_only()
            .enable_http1()
            .build();

        let client = hyper::Client::builder().build(https);
        let resp = client.request(req).await;

        match resp {
            Ok(mut resp) => {
                let bytes = hyper::body::to_bytes(resp.body_mut()).await.unwrap();
                let result = String::from_utf8(bytes.into_iter().collect()).expect("");
                match serde_json::from_str(&result) {
                    Ok(data) => Ok(data),
                    Err(err) => Err(format!("{:?}", err)),
                }
            }
            Err(err) => Err(format!("{:?}", err)),
        }
    }

    async fn bid_get_bid(&self, id: &str) -> Result<ApiResultBid, String> {
        let token = &self.token;
        let mut builder = hyper::Request::builder()
            .method(Method::GET)
            .header(header::AUTHORIZATION, format!("Bearer {token}"));

        let uri_str = format!("{}/api/v1/bid/{id}", self.base_path);
        let uri: hyper::Uri = uri_str.parse().unwrap();

        builder = builder.uri(uri);
        let req = builder.body(hyper::Body::empty()).unwrap();

        let https = hyper_rustls::HttpsConnectorBuilder::new()
            .with_native_roots()
            .https_only()
            .enable_http1()
            .build();

        let client = hyper::Client::builder().build(https);
        let resp = client.request(req).await;

        match resp {
            Ok(mut resp) => {
                let bytes = hyper::body::to_bytes(resp.body_mut()).await.unwrap();
                let result = String::from_utf8(bytes.into_iter().collect()).expect("");
                match serde_json::from_str(&result) {
                    Ok(data) => Ok(data),
                    Err(err) => Err(format!("{:?}", err)),
                }
            }
            Err(err) => Err(format!("{:?}", err)),
        }
    }

    async fn bid_get_bid_summaries(
        &self,
        request_bid_status_code: i32,
        request_start_date: String,
        request_end_date: String,
        request_page_size: i32,
        request_page_nr: i32,
    ) -> Result<ApiResultBids, String> {
        let token = &self.token;
        let mut builder = hyper::Request::builder()
            .method(Method::GET)
            .header(header::AUTHORIZATION, format!("Bearer {token}"));

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.append_pair(
                "request.bidStatusCode",
                &request_bid_status_code.to_string(),
            );
            query.append_pair("request.startDate", &request_start_date.to_string());
            query.append_pair("request.endDate", &request_end_date.to_string());
            query.append_pair("request.pageSize", &request_page_size.to_string());
            query.append_pair("request.pageNr", &request_page_nr.to_string());
            query.finish()
        };
        let uri_str = format!("{}/api/v1/bids?{}", self.base_path, query_string);
        let uri: hyper::Uri = uri_str.parse().unwrap();

        builder = builder.uri(uri);
        let req = builder.body(hyper::Body::empty()).unwrap();

        let https = hyper_rustls::HttpsConnectorBuilder::new()
            .with_native_roots()
            .https_only()
            .enable_http1()
            .build();

        let client = hyper::Client::builder().build(https);
        let resp = client.request(req).await;

        match resp {
            Ok(mut resp) => {
                let bytes = hyper::body::to_bytes(resp.body_mut()).await.unwrap();
                let result = String::from_utf8(bytes.into_iter().collect()).expect("");
                match serde_json::from_str(&result) {
                    Ok(data) => Ok(data),
                    Err(err) => Err(format!("{:?}", err)),
                }
            }
            Err(err) => Err(format!("{:?}", err)),
        }
    }

    async fn bid_make_bids(
        &self,
        bid_request: crate::models::BidRequest,
    ) -> Result<ApiResultMakeBids, String> {
        let token = &self.token;
        let mut builder = hyper::Request::builder()
            .method(Method::POST)
            .header(header::AUTHORIZATION, format!("Bearer {token}"))
            .header(header::CONTENT_TYPE, "application/json");

        let uri_str = format!("{}/api/v1/bid", self.base_path);
        let uri: hyper::Uri = uri_str.parse().unwrap();

        let serialized = serde_json::to_string(&bid_request).unwrap();
        builder = builder.uri(uri);
        builder = builder.header(header::CONTENT_LENGTH, serialized.len());

        let req = builder.body(hyper::Body::from(serialized)).unwrap();

        let resp = self.client.request(req).await;

        match resp {
            Ok(mut resp) => {
                let bytes = hyper::body::to_bytes(resp.body_mut()).await.unwrap();
                let result = String::from_utf8(bytes.into_iter().collect()).expect("");
                match serde_json::from_str(&result) {
                    Ok(data) => Ok(data),
                    Err(err) => Err(format!("{:?}", err)),
                }
            }
            Err(err) => Err(format!("{:?}", err)),
        }
    }
}
