use async_trait::async_trait;
use hyper::{self,header,Method};
use serde_json;
use crate::models::ApiResultMyInvestments;
use crate::models::ApiResultMyAccountBalance;
use crate::models::ApiResultEventLog;
use crate::Client;

#[async_trait]
pub trait AccountApi {
    async fn account_get_active(&self, request_loan_issued_date_from: String, request_loan_issued_date_to: String, request_principal_min: f64, request_principal_max: f64, request_interest_min: f64, request_interest_max: f64, request_length_max: i32, request_length_min: i32, request_late_principal_amount_min: f64, request_late_principal_amount_max: f64, request_debt_occured_on_from: String, request_debt_occured_on_to: String, request_debt_occured_on_for_secondary_from: String, request_debt_occured_on_for_secondary_to: String, request_defaulted_date_from: String, request_defaulted_date_to: String, request_rescheduled_from: String, request_rescheduled_to: String, request_sold_date_from: String, request_sold_date_to: String, request_purchase_date_from: String, request_purchase_date_to: String, request_next_payment_date_to: String, request_next_payment_date_from: String, request_last_payment_date_from: String, request_last_payment_date_to: String, request_countries: Vec<String>, request_ratings: Vec<String>, request_credit_score_min: i32, request_credit_score_max: i32, request_user_name: &str, request_loan_status_code: Vec<i32>, request_income_verification_status: i32, request_loan_debt_management_stage: i32, request_loan_debt_management_stage_type: i32, request_loan_debt_management_date_active_from: String, request_loan_debt_management_date_active_to: String, request_auction_bid_type: i32, request_sales_status: i32, request_is_in_repayment: bool, request_page_size: i32, request_page_nr: i32)
     -> Result<ApiResultMyInvestments, String>;
    async fn account_get_balance(&self, )
     -> Result<ApiResultMyAccountBalance,String>;
    async fn account_get_event_log(&self, request_event_date_from: String, request_event_date_to: String, request_event_type: i32, request_ip_address: &str, request_page_size: i32, request_page_nr: i32)
     -> Result<ApiResultEventLog,String>;
}

#[async_trait]
impl AccountApi for Client {

    async fn account_get_active(&self, request_loan_issued_date_from: String, request_loan_issued_date_to: String, request_principal_min: f64, request_principal_max: f64, request_interest_min: f64, request_interest_max: f64, request_length_max: i32, request_length_min: i32, request_late_principal_amount_min: f64, request_late_principal_amount_max: f64, request_debt_occured_on_from: String, request_debt_occured_on_to: String, request_debt_occured_on_for_secondary_from: String, request_debt_occured_on_for_secondary_to: String, request_defaulted_date_from: String, request_defaulted_date_to: String, request_rescheduled_from: String, request_rescheduled_to: String, request_sold_date_from: String, request_sold_date_to: String, request_purchase_date_from: String, request_purchase_date_to: String, request_next_payment_date_to: String, request_next_payment_date_from: String, request_last_payment_date_from: String, request_last_payment_date_to: String, request_countries: Vec<String>, request_ratings: Vec<String>, request_credit_score_min: i32, request_credit_score_max: i32, request_user_name: &str, request_loan_status_code: Vec<i32>, request_income_verification_status: i32, request_loan_debt_management_stage: i32, request_loan_debt_management_stage_type: i32, request_loan_debt_management_date_active_from: String, request_loan_debt_management_date_active_to: String, request_auction_bid_type: i32, request_sales_status: i32, request_is_in_repayment: bool, request_page_size: i32, request_page_nr: i32)
      -> Result<ApiResultMyInvestments, String> {
        
        let token = &self.token;
        let mut builder = hyper::Request::builder()
            .method(Method::GET)
            .header(header::AUTHORIZATION, format!("Bearer {token}"));

        let status_str: String = request_loan_status_code.iter().map( |&id| id.to_string() + ",").collect();
        let query_string = {
            let mut query = url::form_urlencoded::Serializer::new(String::new());
            query.append_pair("request.loanIssuedDateFrom", &request_loan_issued_date_from.to_string());
            query.append_pair("request.loanIssuedDateTo", &request_loan_issued_date_to.to_string());
            query.append_pair("request.principalMin", &request_principal_min.to_string());
            query.append_pair("request.principalMax", &request_principal_max.to_string());
            query.append_pair("request.interestMin", &request_interest_min.to_string());
            query.append_pair("request.interestMax", &request_interest_max.to_string());
            query.append_pair("request.lengthMax", &request_length_max.to_string());
            query.append_pair("request.lengthMin", &request_length_min.to_string());
            query.append_pair("request.latePrincipalAmountMin", &request_late_principal_amount_min.to_string());
            query.append_pair("request.latePrincipalAmountMax", &request_late_principal_amount_max.to_string());
            query.append_pair("request.debtOccuredOnFrom", &request_debt_occured_on_from.to_string());
            query.append_pair("request.debtOccuredOnTo", &request_debt_occured_on_to.to_string());
            query.append_pair("request.debtOccuredOnForSecondaryFrom", &request_debt_occured_on_for_secondary_from.to_string());
            query.append_pair("request.debtOccuredOnForSecondaryTo", &request_debt_occured_on_for_secondary_to.to_string());
            query.append_pair("request.defaultedDateFrom", &request_defaulted_date_from.to_string());
            query.append_pair("request.defaultedDateTo", &request_defaulted_date_to.to_string());
            query.append_pair("request.rescheduledFrom", &request_rescheduled_from.to_string());
            query.append_pair("request.rescheduledTo", &request_rescheduled_to.to_string());
            query.append_pair("request.soldDateFrom", &request_sold_date_from.to_string());
            query.append_pair("request.soldDateTo", &request_sold_date_to.to_string());
            query.append_pair("request.purchaseDateFrom", &request_purchase_date_from.to_string());
            query.append_pair("request.purchaseDateTo", &request_purchase_date_to.to_string());
            query.append_pair("request.nextPaymentDateTo", &request_next_payment_date_to.to_string());
            query.append_pair("request.nextPaymentDateFrom", &request_next_payment_date_from.to_string());
            query.append_pair("request.lastPaymentDateFrom", &request_last_payment_date_from.to_string());
            query.append_pair("request.lastPaymentDateTo", &request_last_payment_date_to.to_string());
            query.append_pair("request.countries", &request_countries.join(",").to_string());
            query.append_pair("request.ratings", &request_ratings.join(",").to_string());
            query.append_pair("request.creditScoreMin", &request_credit_score_min.to_string());
            query.append_pair("request.creditScoreMax", &request_credit_score_max.to_string());
            query.append_pair("request.userName", &request_user_name.to_string());
            query.append_pair("request.loanStatusCode", &status_str);
            query.append_pair("request.incomeVerificationStatus", &request_income_verification_status.to_string());
            query.append_pair("request.loanDebtManagementStage", &request_loan_debt_management_stage.to_string());
            query.append_pair("request.loanDebtManagementStageType", &request_loan_debt_management_stage_type.to_string());
            query.append_pair("request.loanDebtManagementDateActiveFrom", &request_loan_debt_management_date_active_from.to_string());
            query.append_pair("request.loanDebtManagementDateActiveTo", &request_loan_debt_management_date_active_to.to_string());
            query.append_pair("request.auctionBidType", &request_auction_bid_type.to_string());
            query.append_pair("request.salesStatus", &request_sales_status.to_string());
            query.append_pair("request.isInRepayment", &request_is_in_repayment.to_string());
            query.append_pair("request.pageSize", &request_page_size.to_string());
            query.append_pair("request.pageNr", &request_page_nr.to_string());
            query.finish()
        };
        let uri_str = format!("{}/api/v1/account/investments?{}", self.base_path, query_string);
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

    async fn account_get_balance(&self, ) -> Result<ApiResultMyAccountBalance,String> {
        let token = &self.token;
        let mut builder = hyper::Request::builder()
            .method(Method::GET)
            .header(header::AUTHORIZATION, format!("Bearer {token}"));

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.finish()
        };
        let uri_str = format!("{}/api/v1/account/balance?{}", self.base_path, query_string);
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

    async fn account_get_event_log(&self, request_event_date_from: String, request_event_date_to: String, request_event_type: i32, request_ip_address: &str, request_page_size: i32, request_page_nr: i32)
     -> Result<ApiResultEventLog,String> {
        let token = &self.token;
        let mut builder = hyper::Request::builder()
            .method(Method::GET)
            .header(header::AUTHORIZATION, format!("Bearer {token}"));

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.append_pair("request.eventDateFrom", &request_event_date_from.to_string());
            query.append_pair("request.eventDateTo", &request_event_date_to.to_string());
            query.append_pair("request.eventType", &request_event_type.to_string());
            query.append_pair("request.ipAddress", &request_ip_address.to_string());
            query.append_pair("request.pageSize", &request_page_size.to_string());
            query.append_pair("request.pageNr", &request_page_nr.to_string());
            query.finish()
        };
        let uri_str = format!("{}/api/v1/eventlog?{}", self.base_path, query_string);
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
