

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResultReport {
  /// Number of items returned
  #[serde(rename = "Count")]
  count: i32,
  /// The payload of the response. Type depends on the API request.
  #[serde(rename = "Payload")]
  payload: Option<crate::models::Report>,
  /// Indicates if the request was successfull or not.  true if the request was handled successfully, false otherwise.
  #[serde(rename = "Success")]
  success: bool,
  /// Error(s) accociated with the API request.
  #[serde(rename = "Errors")]
  errors: Option<Vec<crate::models::ApiError>>
}

impl ApiResultReport {
  pub fn new(count: i32, success: bool) -> ApiResultReport {
    ApiResultReport {
      count: count,
      payload: None,
      success: success,
      errors: None
    }
  }

  pub fn set_count(&mut self, count: i32) {
    self.count = count;
  }

  pub fn with_count(mut self, count: i32) -> ApiResultReport {
    self.count = count;
    self
  }

  pub fn count(&self) -> &i32 {
    &self.count
  }


  pub fn set_payload(&mut self, payload: crate::models::Report) {
    self.payload = Some(payload);
  }

  pub fn with_payload(mut self, payload: crate::models::Report) -> ApiResultReport {
    self.payload = Some(payload);
    self
  }

  pub fn payload(&self) -> Option<&crate::models::Report> {
    self.payload.as_ref()
  }

  pub fn reset_payload(&mut self) {
    self.payload = None;
  }

  pub fn set_success(&mut self, success: bool) {
    self.success = success;
  }

  pub fn with_success(mut self, success: bool) -> ApiResultReport {
    self.success = success;
    self
  }

  pub fn success(&self) -> &bool {
    &self.success
  }


  pub fn set_errors(&mut self, errors: Vec<crate::models::ApiError>) {
    self.errors = Some(errors);
  }

  pub fn with_errors(mut self, errors: Vec<crate::models::ApiError>) -> ApiResultReport {
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


