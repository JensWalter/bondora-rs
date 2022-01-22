#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResultBids {
  /// Requested Max items in result
  #[serde(rename = "PageSize")]
  page_size: Option<i32>,
  /// Requested page nr
  #[serde(rename = "PageNr")]
  page_nr: Option<i32>,
  /// Total number of items found
  #[serde(rename = "TotalCount")]
  total_count: i32,
  /// Number of items returned
  #[serde(rename = "Count")]
  count: i32,
  /// The payload of the response. Type depends on the API request.
  #[serde(rename = "Payload")]
  payload: Option<Vec<crate::models::BidSummary>>,
  /// Indicates if the request was successfull or not.  true if the request was handled successfully, false otherwise.
  #[serde(rename = "Success")]
  success: bool,
  /// Error(s) accociated with the API request.
  #[serde(rename = "Errors")]
  errors: Option<Vec<crate::models::ApiError>>
}

impl ApiResultBids {
  pub fn new(total_count: i32, count: i32, success: bool) -> ApiResultBids {
    ApiResultBids {
      page_size: None,
      page_nr: None,
      total_count: total_count,
      count: count,
      payload: None,
      success: success,
      errors: None
    }
  }

  pub fn set_page_size(&mut self, page_size: i32) {
    self.page_size = Some(page_size);
  }

  pub fn with_page_size(mut self, page_size: i32) -> ApiResultBids {
    self.page_size = Some(page_size);
    self
  }

  pub fn page_size(&self) -> Option<&i32> {
    self.page_size.as_ref()
  }

  pub fn reset_page_size(&mut self) {
    self.page_size = None;
  }

  pub fn set_page_nr(&mut self, page_nr: i32) {
    self.page_nr = Some(page_nr);
  }

  pub fn with_page_nr(mut self, page_nr: i32) -> ApiResultBids {
    self.page_nr = Some(page_nr);
    self
  }

  pub fn page_nr(&self) -> Option<&i32> {
    self.page_nr.as_ref()
  }

  pub fn reset_page_nr(&mut self) {
    self.page_nr = None;
  }

  pub fn set_total_count(&mut self, total_count: i32) {
    self.total_count = total_count;
  }

  pub fn with_total_count(mut self, total_count: i32) -> ApiResultBids {
    self.total_count = total_count;
    self
  }

  pub fn total_count(&self) -> &i32 {
    &self.total_count
  }


  pub fn set_count(&mut self, count: i32) {
    self.count = count;
  }

  pub fn with_count(mut self, count: i32) -> ApiResultBids {
    self.count = count;
    self
  }

  pub fn count(&self) -> &i32 {
    &self.count
  }


  pub fn set_payload(&mut self, payload: Vec<crate::models::BidSummary>) {
    self.payload = Some(payload);
  }

  pub fn with_payload(mut self, payload: Vec<crate::models::BidSummary>) -> ApiResultBids {
    self.payload = Some(payload);
    self
  }

  pub fn payload(&self) -> Option<&Vec<crate::models::BidSummary>> {
    self.payload.as_ref()
  }

  pub fn reset_payload(&mut self) {
    self.payload = None;
  }

  pub fn set_success(&mut self, success: bool) {
    self.success = success;
  }

  pub fn with_success(mut self, success: bool) -> ApiResultBids {
    self.success = success;
    self
  }

  pub fn success(&self) -> &bool {
    &self.success
  }


  pub fn set_errors(&mut self, errors: Vec<crate::models::ApiError>) {
    self.errors = Some(errors);
  }

  pub fn with_errors(mut self, errors: Vec<crate::models::ApiError>) -> ApiResultBids {
    self.errors = Some(errors);
    self
  }

  pub fn errors(&self) -> Option<&Vec<crate::models::ApiError>> {
    self.errors.as_ref()
  }

  pub fn reset_errors(&mut self) {
    self.errors = None;
  }

}



