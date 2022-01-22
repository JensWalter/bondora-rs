
/// ApiResult : Result of the API request

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResult {
  /// Indicates if the request was successfull or not.  true if the request was handled successfully, false otherwise.
  #[serde(rename = "Success")]
  success: bool,
  /// Error(s) accociated with the API request.
  #[serde(rename = "Errors")]
  errors: Option<Vec<crate::models::ApiError>>
}

impl ApiResult {
  /// Result of the API request
  pub fn new(success: bool) -> ApiResult {
    ApiResult {
      success: success,
      errors: None
    }
  }

  pub fn set_success(&mut self, success: bool) {
    self.success = success;
  }

  pub fn with_success(mut self, success: bool) -> ApiResult {
    self.success = success;
    self
  }

  pub fn success(&self) -> &bool {
    &self.success
  }


  pub fn set_errors(&mut self, errors: Vec<crate::models::ApiError>) {
    self.errors = Some(errors);
  }

  pub fn with_errors(mut self, errors: Vec<crate::models::ApiError>) -> ApiResult {
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



