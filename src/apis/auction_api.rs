use crate::models::ApiResultAuctions;
use crate::models::ApiResultExtendedAuction;
use crate::models::AuctionRequest;
use crate::Client;
use async_trait::async_trait;
use hyper;
use hyper::{header, Method};
use serde_json;

#[async_trait]
pub trait AuctionApi {
    async fn auction_get(&self, id: &str) -> Result<ApiResultExtendedAuction, String>;
    async fn auction_get_active(
        &self,
        options: AuctionRequest,
    ) -> Result<ApiResultAuctions, String>;
}

#[async_trait]
impl AuctionApi for Client {
    async fn auction_get(&self, id: &str) -> Result<ApiResultExtendedAuction, String> {
        let token = &self.token;
        let mut builder = hyper::Request::builder()
            .method(Method::GET)
            .header(header::AUTHORIZATION, format!("Bearer {token}"));

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.finish()
        };
        let uri_str = format!(
            "{}/api/v1/auction/{id}?{}",
            self.base_path,
            query_string,
            id = id
        );
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
                    Err(err) => Err(format!("{:?}", err)),
                }
            }
            Err(err) => Err(format!("{:?}", err)),
        }
    }

    async fn auction_get_active(
        &self,
        options: AuctionRequest,
    ) -> Result<ApiResultAuctions, String> {
        let token = &self.token;
        let mut builder = hyper::Request::builder()
            .method(Method::GET)
            .header(header::AUTHORIZATION, format!("Bearer {token}"));

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            if let Some(x) = &options.countries {
                query.append_pair("request.countries", &x.join(","));
            }
            if let Some(x) = &options.ratings {
                query.append_pair("request.ratings", &x.join(","));
            }
            if let Some(x) = &options.gender {
                query.append_pair("request.gender", &x.to_string());
            }
            if let Some(x) = &options.sum_min {
                query.append_pair("sumMin", &x.to_string());
            }
            if let Some(x) = &options.sum_max {
                query.append_pair("sumMax", &x.to_string());
            }
            if let Some(x) = &options.terms {
                let terms_str: String = x.iter().map(|&id| id.to_string() + ",").collect();
                query.append_pair("request.terms", &terms_str);
            }
            if let Some(x) = &options.age_min {
                query.append_pair("request.ageMin", &x.to_string());
            }
            if let Some(x) = &options.age_max {
                query.append_pair("request.ageMax", &x.to_string());
            }
            if let Some(x) = &options.loan_number {
                query.append_pair("request.loanNumber", &x.to_string());
            }
            if let Some(x) = &options.user_name {
                query.append_pair("request.userName", &x.to_string());
            }
            if let Some(x) = &options.application_date_from {
                query.append_pair("request.applicationDateFrom", &x.to_string());
            }
            if let Some(x) = &options.application_date_to {
                query.append_pair("request.applicationDateTo", &x.to_string());
            }
            if let Some(x) = &options.credit_score_min {
                query.append_pair("request.creditScoreMin", &x.to_string());
            }
            if let Some(x) = &options.credit_score_max {
                query.append_pair("request.creditScoreMax", &x.to_string());
            }
            if let Some(x) = &options.credit_scores_ee_mini {
                query.append_pair("request.creditScoresEeMini", &x.join(","));
            }
            if let Some(x) = &options.interest_min {
                query.append_pair("request.interestMin", &x.to_string());
            }
            if let Some(x) = &options.interest_max {
                query.append_pair("request.interestMax", &x.to_string());
            }
            if let Some(x) = &options.income_total_min {
                query.append_pair("request.incomeTotalMin", &x.to_string());
            }
            if let Some(x) = &options.income_total_max {
                query.append_pair("request.incomeTotalMax", &x.to_string());
            }
            if let Some(x) = &options.model_version {
                query.append_pair("request.modelVersion", &x.to_string());
            }
            if let Some(x) = &options.expected_loss_min {
                query.append_pair("request.expectedLossMin", &x.to_string());
            }
            if let Some(x) = &options.expected_loss_max {
                query.append_pair("request.expectedLossMax", &x.to_string());
            }
            if let Some(x) = &options.listed_on_utc_from {
                query.append_pair("request.listedOnUTCFrom", &x.to_string());
            }
            if let Some(x) = &options.listed_on_utc_to {
                query.append_pair("request.listedOnUTCTo", &x.to_string());
            }
            if let Some(x) = &options.page_size {
                query.append_pair("request.pageSize", &x.to_string());
            }
            if let Some(x) = &options.page_nr {
                query.append_pair("request.pageNr", &x.to_string());
            }
            query.finish()
        };
        let uri_str = format!("{}/api/v1/auctions?{}", self.base_path, query_string);
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
                    Err(err) => Err(format!("{:?} response payload= {}", err, result)),
                }
            }
            Err(err) => Err(format!("{:?}", err)),
        }
    }
}
