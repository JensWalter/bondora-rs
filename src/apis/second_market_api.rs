use async_trait::async_trait;
use hyper;
use serde_json;
use crate::Client;
use crate::models::ApiResult;
use crate::models::ApiResultLoanPartDetails;
use crate::models::ApiResultSecondMarket;
use crate::models::ApiResultSecondMarketItemSummaryList;
use crate::models::ApiResultSecondMarketItemSummary;
use crate::models::ApiResultLoanPartDetailsList;
use crate::models::ApiResultSecondMarketSale;
use hyper::{Method, header};

#[async_trait]
pub trait SecondMarketApi {
    async fn second_market_buy(&self, buy_request: crate::models::SecondMarketBuyRequest)
     -> Result<ApiResult,String>;
    async fn second_market_cancel(&self, id: &str)
     -> Result<ApiResult,String>;
    async fn second_market_cancel_multiple(&self, cancel_request: crate::models::SecondMarketCancelRequest)
     -> Result<ApiResult,String>;
    async fn second_market_get(&self, id: &str)
     -> Result<ApiResultLoanPartDetails,String>;
    async fn second_market_get_active(&self, request_loan_issued_date_from: String, request_loan_issued_date_to: String, request_principal_min: f64, request_principal_max: f64, request_interest_min: f64, request_interest_max: f64, request_length_max: i32, request_length_min: i32, request_has_debt: bool, request_loan_status_code: Vec<i32>, request_loan_debt_management_stage_type: i32, request_loan_debt_management_date_active_from: String, request_loan_debt_management_date_active_to: String, request_late_principal_amount_min: f64, request_late_principal_amount_max: f64, request_price_min: f64, request_price_max: f64, request_use_of_loan: i32, request_has_new_schedule: bool, request_countries: Vec<String>, request_ratings: Vec<String>, request_credit_score_min: i32, request_credit_score_max: i32, request_user_name: &str, request_gender: i32, request_age_min: i32, request_age_max: i32, request_income_verification_status: i32, request_show_my_items: bool, request_auction_id: &str, request_listed_on_date_from: String, request_listed_on_date_to: String, request_debt_occured_on_from: String, request_debt_occured_on_to: String, request_debt_occured_on_for_secondary_from: String, request_debt_occured_on_for_secondary_to: String, request_defaulted_date_from: String, request_defaulted_date_to: String, request_rescheduled_from: String, request_rescheduled_to: String, request_last_payment_date_from: String, request_last_payment_date_to: String, request_next_payment_date_from: String, request_next_payment_date_to: String, request_desired_discount_rate_min: f64, request_desired_discount_rate_max: f64, request_xirr_min: f64, request_xirr_max: f64, request_page_size: i32, request_page_nr: i32)
     -> Result<ApiResultSecondMarket,String>;
    async fn second_market_get_item(&self, id: &str)
     -> Result<ApiResultSecondMarketItemSummary,String>;
    async fn second_market_get_item_list(&self, request_item_ids: Vec<String>)
     -> Result<ApiResultSecondMarketItemSummaryList,String>;
    async fn second_market_get_item_list2(&self, request: crate::models::SecondMarketListingRequest)
     -> Result<ApiResultSecondMarketItemSummaryList,String>;
    async fn second_market_get_list(&self, request_item_ids: Vec<String>)
     -> Result<ApiResultLoanPartDetailsList,String>;
    async fn second_market_get_list2(&self, request: crate::models::LoanPartDetailsRequest)
     -> Result<ApiResultLoanPartDetailsList,String>;
    async fn second_market_sell(&self, sale_request: crate::models::SecondMarketSaleRequest)
     -> Result<ApiResultSecondMarketSale,String>;
}

#[async_trait]
impl SecondMarketApi for Client {
    async fn second_market_buy(&self, buy_request: crate::models::SecondMarketBuyRequest)
     -> Result<ApiResult,String> {

        let token = &self.token;
        let mut builder = hyper::Request::builder()
            .method(Method::POST)
            .header(header::AUTHORIZATION, format!("Bearer {token}"))
            .header(header::CONTENT_TYPE, "application/json");

        let uri_str = format!("{}/api/v1/secondarymarket/buy", self.base_path);
        let uri: hyper::Uri = uri_str.parse().unwrap();

        let serialized = serde_json::to_string(&buy_request).unwrap();
        builder = builder.header(header::CONTENT_LENGTH,serialized.len());
        builder = builder.uri(uri);

        let req = builder.body(hyper::Body::from(serialized)).unwrap();

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

    async fn second_market_cancel(&self, id: &str) -> Result<ApiResult,String> {

        let token = &self.token;
        let mut builder = hyper::Request::builder()
            .method(Method::POST)
            .header(header::AUTHORIZATION, format!("Bearer {token}"))
            .header(header::CONTENT_TYPE, "application/json");

        let uri_str = format!("{}/api/v1/secondarymarket/{id}/cancel", self.base_path);
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

    async fn second_market_cancel_multiple(&self, cancel_request: crate::models::SecondMarketCancelRequest) -> Result<ApiResult,String> {
        
        let token = &self.token;
        let mut builder = hyper::Request::builder()
            .method(Method::POST)
            .header(header::AUTHORIZATION, format!("Bearer {token}"))
            .header(header::CONTENT_TYPE, "application/json");

        let uri_str = format!("{}/api/v1/secondarymarket/cancel", self.base_path);
        let uri: hyper::Uri = uri_str.parse().unwrap();

        let serialized = serde_json::to_string(&cancel_request).unwrap();
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

    async fn second_market_get(&self, id: &str) -> Result<ApiResultLoanPartDetails,String> {

        let mut builder = hyper::Request::builder()
            .method(Method::GET);

        let uri_str = format!("{}/api/v1/loanpart/{id}", self.base_path);
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

    async fn second_market_get_active(&self, request_loan_issued_date_from: String, request_loan_issued_date_to: String, request_principal_min: f64, request_principal_max: f64, request_interest_min: f64, request_interest_max: f64, request_length_max: i32, request_length_min: i32, request_has_debt: bool, request_loan_status_code: Vec<i32>, request_loan_debt_management_stage_type: i32, request_loan_debt_management_date_active_from: String, request_loan_debt_management_date_active_to: String, request_late_principal_amount_min: f64, request_late_principal_amount_max: f64, request_price_min: f64, request_price_max: f64, request_use_of_loan: i32, request_has_new_schedule: bool, request_countries: Vec<String>, request_ratings: Vec<String>, request_credit_score_min: i32, request_credit_score_max: i32, request_user_name: &str, request_gender: i32, request_age_min: i32, request_age_max: i32, request_income_verification_status: i32, request_show_my_items: bool, request_auction_id: &str, request_listed_on_date_from: String, request_listed_on_date_to: String, request_debt_occured_on_from: String, request_debt_occured_on_to: String, request_debt_occured_on_for_secondary_from: String, request_debt_occured_on_for_secondary_to: String, request_defaulted_date_from: String, request_defaulted_date_to: String, request_rescheduled_from: String, request_rescheduled_to: String, request_last_payment_date_from: String, request_last_payment_date_to: String, request_next_payment_date_from: String, request_next_payment_date_to: String, request_desired_discount_rate_min: f64, request_desired_discount_rate_max: f64, request_xirr_min: f64, request_xirr_max: f64, request_page_size: i32, request_page_nr: i32)
     -> Result<ApiResultSecondMarket,String> {
        
        let mut builder = hyper::Request::builder()
            .method(Method::GET);

        let status_str: String = request_loan_status_code.iter().map( |&id| id.to_string() + ",").collect();
        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.append_pair("request.loanIssuedDateFrom", &request_loan_issued_date_from.to_string());
            query.append_pair("request.loanIssuedDateTo", &request_loan_issued_date_to.to_string());
            query.append_pair("request.principalMin", &request_principal_min.to_string());
            query.append_pair("request.principalMax", &request_principal_max.to_string());
            query.append_pair("request.interestMin", &request_interest_min.to_string());
            query.append_pair("request.interestMax", &request_interest_max.to_string());
            query.append_pair("request.lengthMax", &request_length_max.to_string());
            query.append_pair("request.lengthMin", &request_length_min.to_string());
            query.append_pair("request.hasDebt", &request_has_debt.to_string());
            query.append_pair("request.loanStatusCode", &status_str);
            query.append_pair("request.loanDebtManagementStageType", &request_loan_debt_management_stage_type.to_string());
            query.append_pair("request.loanDebtManagementDateActiveFrom", &request_loan_debt_management_date_active_from.to_string());
            query.append_pair("request.loanDebtManagementDateActiveTo", &request_loan_debt_management_date_active_to.to_string());
            query.append_pair("request.latePrincipalAmountMin", &request_late_principal_amount_min.to_string());
            query.append_pair("request.latePrincipalAmountMax", &request_late_principal_amount_max.to_string());
            query.append_pair("request.priceMin", &request_price_min.to_string());
            query.append_pair("request.priceMax", &request_price_max.to_string());
            query.append_pair("request.useOfLoan", &request_use_of_loan.to_string());
            query.append_pair("request.hasNewSchedule", &request_has_new_schedule.to_string());
            query.append_pair("request.countries", &request_countries.join(",").to_string());
            query.append_pair("request.ratings", &request_ratings.join(",").to_string());
            query.append_pair("request.creditScoreMin", &request_credit_score_min.to_string());
            query.append_pair("request.creditScoreMax", &request_credit_score_max.to_string());
            query.append_pair("request.userName", &request_user_name.to_string());
            query.append_pair("request.gender", &request_gender.to_string());
            query.append_pair("request.ageMin", &request_age_min.to_string());
            query.append_pair("request.ageMax", &request_age_max.to_string());
            query.append_pair("request.incomeVerificationStatus", &request_income_verification_status.to_string());
            query.append_pair("request.showMyItems", &request_show_my_items.to_string());
            query.append_pair("request.auctionId", &request_auction_id.to_string());
            query.append_pair("request.listedOnDateFrom", &request_listed_on_date_from.to_string());
            query.append_pair("request.listedOnDateTo", &request_listed_on_date_to.to_string());
            query.append_pair("request.debtOccuredOnFrom", &request_debt_occured_on_from.to_string());
            query.append_pair("request.debtOccuredOnTo", &request_debt_occured_on_to.to_string());
            query.append_pair("request.debtOccuredOnForSecondaryFrom", &request_debt_occured_on_for_secondary_from.to_string());
            query.append_pair("request.debtOccuredOnForSecondaryTo", &request_debt_occured_on_for_secondary_to.to_string());
            query.append_pair("request.defaultedDateFrom", &request_defaulted_date_from.to_string());
            query.append_pair("request.defaultedDateTo", &request_defaulted_date_to.to_string());
            query.append_pair("request.rescheduledFrom", &request_rescheduled_from.to_string());
            query.append_pair("request.rescheduledTo", &request_rescheduled_to.to_string());
            query.append_pair("request.lastPaymentDateFrom", &request_last_payment_date_from.to_string());
            query.append_pair("request.lastPaymentDateTo", &request_last_payment_date_to.to_string());
            query.append_pair("request.nextPaymentDateFrom", &request_next_payment_date_from.to_string());
            query.append_pair("request.nextPaymentDateTo", &request_next_payment_date_to.to_string());
            query.append_pair("request.desiredDiscountRateMin", &request_desired_discount_rate_min.to_string());
            query.append_pair("request.desiredDiscountRateMax", &request_desired_discount_rate_max.to_string());
            query.append_pair("request.xirrMin", &request_xirr_min.to_string());
            query.append_pair("request.xirrMax", &request_xirr_max.to_string());
            query.append_pair("request.pageSize", &request_page_size.to_string());
            query.append_pair("request.pageNr", &request_page_nr.to_string());
            query.finish()
        };
        let uri_str = format!("{}/api/v1/secondarymarket?{}", self.base_path, query_string);
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

    async fn second_market_get_item(&self, id: &str) -> Result<ApiResultSecondMarketItemSummary,String> {
        
        let mut builder = hyper::Request::builder()
            .method(Method::GET);

        let uri_str = format!("{}/api/v1/secondarymarket/{id}", self.base_path);
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

    async fn second_market_get_item_list(&self, request_item_ids: Vec<String>) -> Result<ApiResultSecondMarketItemSummaryList,String> {
        
        let mut builder = hyper::Request::builder()
            .method(Method::GET);

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.append_pair("request.itemIds", &request_item_ids.join(",").to_string());
            query.finish()
        };
        let uri_str = format!("{}/api/v1/secondarymarket/list?{}", self.base_path, query_string);
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

    async fn second_market_get_item_list2(&self, request: crate::models::SecondMarketListingRequest) -> Result<ApiResultSecondMarketItemSummaryList,String> {
        
        let token = &self.token;
        let mut builder = hyper::Request::builder()
            .method(Method::POST)
            .header(header::AUTHORIZATION, format!("Bearer {token}"))
            .header(header::CONTENT_TYPE, "application/json");

        let uri_str = format!("{}/api/v1/secondarymarket/list", self.base_path);
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

    async fn second_market_get_list(&self, request_item_ids: Vec<String>) -> Result<ApiResultLoanPartDetailsList,String> {
        
        let mut builder = hyper::Request::builder()
            .method(Method::GET);

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.append_pair("request.itemIds", &request_item_ids.join(",").to_string());
            query.finish()
        };
        let uri_str = format!("{}/api/v1/loanpart/list?{}", self.base_path, query_string);
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

    async fn second_market_get_list2(&self, request: crate::models::LoanPartDetailsRequest) -> Result<ApiResultLoanPartDetailsList,String> {
        
        let token = &self.token;
        let mut builder = hyper::Request::builder()
            .method(Method::POST)
            .header(header::AUTHORIZATION, format!("Bearer {token}"))
            .header(header::CONTENT_TYPE, "application/json");

        let uri_str = format!("{}/api/v1/loanpart/list", self.base_path);
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

    async fn second_market_sell(&self, sale_request: crate::models::SecondMarketSaleRequest) -> Result<ApiResultSecondMarketSale,String> {
        
        let token = &self.token;
        let mut builder = hyper::Request::builder()
            .method(Method::POST)
            .header(header::AUTHORIZATION, format!("Bearer {token}"))
            .header(header::CONTENT_TYPE, "application/json");

        let uri_str = format!("{}/api/v1/secondarymarket/sell", self.base_path);
        let uri: hyper::Uri = uri_str.parse().unwrap();

        let serialized = serde_json::to_string(&sale_request).unwrap();
        builder = builder.header(header::CONTENT_LENGTH,serialized.len());
        builder = builder.uri(uri);

        let req = builder.body(hyper::Body::from(serialized)).unwrap();

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
