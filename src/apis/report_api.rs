use async_trait::async_trait;
use hyper;
use serde_json;
use crate::Client;
use crate::models::ApiResultReport;
use crate::models::ApiResultCreateReport;
use crate::models::ApiResultReportList;
use crate::models::ApiResultPublicDataset;
use hyper::{Method, header};

#[async_trait]
pub trait ReportApi {
    async fn report_generate_report(&self, request: crate::models::ReportCreateRequest) -> Result<ApiResultCreateReport,String>;
    async fn report_get_public_dataset(&self, request_loan_ids: Vec<String>, request_countries: Vec<String>, request_ratings: Vec<String>, request_loan_date_from: String, request_loan_date_to: String, request_page_size: i32, request_page_nr: i32)
     -> Result<ApiResultPublicDataset,String>;
    async fn report_get_report(&self, id: &str) -> Result<ApiResultReport,String>;
    async fn report_get_report_list(&self, ) -> Result<ApiResultReportList,String>;
}

#[async_trait]
impl ReportApi for Client {
    async fn report_generate_report(&self, request: crate::models::ReportCreateRequest) -> Result<ApiResultCreateReport,String> {
        
        let token = &self.token;
        let mut builder = hyper::Request::builder()
            .method(Method::POST)
            .header(header::AUTHORIZATION, format!("Bearer {token}"))
            .header(header::CONTENT_TYPE, "application/json");

        let uri_str = format!("{}/api/v1/report", self.base_path);
        let uri: hyper::Uri = uri_str.parse().unwrap();

        let serialized = serde_json::to_string(&request).unwrap();
        builder = builder.header(header::CONTENT_LENGTH,serialized.len());
        builder = builder.uri(uri);

        let req = builder.body(hyper::Body::from(serialized)).unwrap();

        let https = hyper_rustls::HttpsConnectorBuilder::new()
        .with_native_roots()
        .https_only()
        .enable_http1().build();

        let client = hyper::Client::builder().build(https);
        let resp = client.request(req).await;

        match resp {
            Ok(mut resp) => {
                let bytes = hyper::body::to_bytes(resp.body_mut()).await.unwrap();
                let result = String::from_utf8(bytes.into_iter().collect()).expect("");
                match serde_json::from_str(&result) {
                    Ok(data) => Ok(data),
                    Err(err) => Err(format!("{:?}",err)),
                }
            }
            Err(err) => Err(format!("{:?}",err)),
        }
    }

    async fn report_get_public_dataset(&self, request_loan_ids: Vec<String>, request_countries: Vec<String>, request_ratings: Vec<String>, request_loan_date_from: String, request_loan_date_to: String, request_page_size: i32, request_page_nr: i32)
     -> Result<ApiResultPublicDataset,String> {

        let token = &self.token;
        let mut builder = hyper::Request::builder()
            .method(Method::GET)
            .header(header::AUTHORIZATION, format!("Bearer {token}"));

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.append_pair("request.loanIds", &request_loan_ids.join(",").to_string());
            query.append_pair("request.countries", &request_countries.join(",").to_string());
            query.append_pair("request.ratings", &request_ratings.join(",").to_string());
            query.append_pair("request.loanDateFrom", &request_loan_date_from.to_string());
            query.append_pair("request.loanDateTo", &request_loan_date_to.to_string());
            query.append_pair("request.pageSize", &request_page_size.to_string());
            query.append_pair("request.pageNr", &request_page_nr.to_string());
            query.finish()
        };
        let uri_str = format!("{}/api/v1/publicdataset?{}", self.base_path, query_string);
        let uri: hyper::Uri = uri_str.parse().unwrap();

        builder = builder.uri(uri);
        let req = builder.body(hyper::Body::empty()).unwrap();

        let https = hyper_rustls::HttpsConnectorBuilder::new()
            .with_native_roots()
            .https_only()
            .enable_http1().build();

        let client = hyper::Client::builder().build(https);
        let resp = client.request(req).await;

        match resp {
            Ok(mut resp) => {
                let bytes = hyper::body::to_bytes(resp.body_mut()).await.unwrap();
                let result = String::from_utf8(bytes.into_iter().collect()).expect("");
                match serde_json::from_str(&result) {
                    Ok(data) => Ok(data),
                    Err(err) => Err(format!("{:?}",err)),
                }
            }
            Err(err) => Err(format!("{:?}",err)),
        }
    }

    async fn report_get_report(&self, id: &str) -> Result<ApiResultReport,String> {

        let token = &self.token;
        let mut builder = hyper::Request::builder()
            .method(Method::GET)
            .header(header::AUTHORIZATION, format!("Bearer {token}"));

        let uri_str = format!("{}/api/v1/report/{id}", self.base_path);
        let uri: hyper::Uri = uri_str.parse().unwrap();

        builder = builder.uri(uri);
        let req = builder.body(hyper::Body::empty()).unwrap();

        let https = hyper_rustls::HttpsConnectorBuilder::new()
            .with_native_roots()
            .https_only()
            .enable_http1().build();

        let client = hyper::Client::builder().build(https);
        let resp = client.request(req).await;

        match resp {
            Ok(mut resp) => {
                let bytes = hyper::body::to_bytes(resp.body_mut()).await.unwrap();
                let result = String::from_utf8(bytes.into_iter().collect()).expect("");
                match serde_json::from_str(&result) {
                    Ok(data) => Ok(data),
                    Err(err) => Err(format!("{:?}",err)),
                }
            }
            Err(err) => Err(format!("{:?}",err)),
        }
    }

    async fn report_get_report_list(&self, ) -> Result<ApiResultReportList,String> {

        let token = &self.token;
        let mut builder = hyper::Request::builder()
            .method(Method::GET)
            .header(header::AUTHORIZATION, format!("Bearer {token}"));

        let uri_str = format!("{}/api/v1/reports", self.base_path);
        let uri: hyper::Uri = uri_str.parse().unwrap();

        builder = builder.uri(uri);
        let req = builder.body(hyper::Body::empty()).unwrap();

        let resp = self.client.request(req).await;

        match resp {
            Ok(mut resp) => {
                let bytes = hyper::body::to_bytes(resp.body_mut()).await.unwrap();
                let result = String::from_utf8(bytes.into_iter().collect()).expect("");
                match serde_json::from_str(&result) {
                    Ok(data) => Ok(data),
                    Err(err) => Err(format!("{:?}",err)),
                }
            }
            Err(err) => Err(format!("{:?}",err)),
        }
    }

}
