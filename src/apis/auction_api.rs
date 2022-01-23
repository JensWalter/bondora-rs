use hyper;
use serde_json;
use crate::apis::client::APIClient;
use crate::models::ApiResultExtendedAuction;
use crate::models::ApiResultAuctions;
use crate::models::AuctionRequest;
use hyper::{Method,header};
use async_trait::async_trait;

#[async_trait]
pub trait AuctionApi {
    async fn auction_get(&self, id: &str) -> Result<ApiResultExtendedAuction,String>;
    async fn auction_get_active(&self, options: AuctionRequest)
     -> Result<ApiResultAuctions,String>;
}

#[async_trait]
impl AuctionApi for APIClient {
    async fn auction_get(&self, id: &str) -> Result<ApiResultExtendedAuction,String> {

        let token = &self.configuration.token;
        let mut builder = hyper::Request::builder()
            .method(Method::GET)
            .header(header::AUTHORIZATION, format!("Bearer {token}"));

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.finish()
        };
        let uri_str = format!("{}/api/v1/auction/{id}?{}", self.configuration.base_path, query_string, id=id);
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

    async fn auction_get_active(&self, options: AuctionRequest)
     -> Result<ApiResultAuctions,String> {

        let token = &self.configuration.token;
        let mut builder = hyper::Request::builder()
            .method(Method::GET)
            .header(header::AUTHORIZATION, format!("Bearer {token}"));
        
        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            if let Some(x) = &options.countries {
                query.append_pair("request.countries", &x.join(",").to_string());
            }
            if let Some(x) = &options.ratings {
                query.append_pair("request.ratings", &x.join(",").to_string());
            }
            if let Some(x) = &options.gender {
                query.append_pair("request.gender", &x.to_string());
            }

            // query.append_pair("request.sumMin", &request_sum_min.to_string());
            // query.append_pair("request.sumMax", &request_sum_max.to_string());
            if let Some(x) = &options.terms {
                let terms_str: String = x.iter().map( |&id| id.to_string() + ",").collect();
                query.append_pair("request.terms", &terms_str);
            }

            // query.append_pair("request.ageMin", &request_age_min.to_string());
            // query.append_pair("request.ageMax", &request_age_max.to_string());
            // query.append_pair("request.loanNumber", &request_loan_number.to_string());
            // query.append_pair("request.userName", &request_user_name.to_string());
            // query.append_pair("request.applicationDateFrom", &request_application_date_from.to_string());
            // query.append_pair("request.applicationDateTo", &request_application_date_to.to_string());
            // query.append_pair("request.creditScoreMin", &request_credit_score_min.to_string());
            // query.append_pair("request.creditScoreMax", &request_credit_score_max.to_string());
            // query.append_pair("request.creditScoresEeMini", &request_credit_scores_ee_mini.join(",").to_string());
            // query.append_pair("request.interestMin", &request_interest_min.to_string());
            // query.append_pair("request.interestMax", &request_interest_max.to_string());
            // query.append_pair("request.incomeTotalMin", &request_income_total_min.to_string());
            // query.append_pair("request.incomeTotalMax", &request_income_total_max.to_string());
            // query.append_pair("request.modelVersion", &request_model_version.to_string());
            // query.append_pair("request.expectedLossMin", &request_expected_loss_min.to_string());
            // query.append_pair("request.expectedLossMax", &request_expected_loss_max.to_string());
            // query.append_pair("request.listedOnUTCFrom", &request_listed_on_utc_from.to_string());
            // query.append_pair("request.listedOnUTCTo", &request_listed_on_utc_to.to_string());
            // query.append_pair("request.pageSize", &request_page_size.to_string());
            // query.append_pair("request.pageNr", &request_page_nr.to_string());
            query.finish()
        };
        let uri_str = format!("{}/api/v1/auctions?{}", self.configuration.base_path, query_string);
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

}
