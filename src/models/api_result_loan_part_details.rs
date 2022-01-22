
/// ApiResultLoanPartDetails : Returns a single LoanPartDetails

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResultLoanPartDetails {
  /// The payload of the response. Type depends on the API request.
  #[serde(rename = "Payload")]
  payload: Option<crate::models::LoanPartDetails>,
  /// Indicates if the request was successfull or not.  true if the request was handled successfully, false otherwise.
  #[serde(rename = "Success")]
  success: bool,
  /// Error(s) accociated with the API request.
  #[serde(rename = "Errors")]
  errors: Option<Vec<crate::models::ApiError>>
}

impl ApiResultLoanPartDetails {
  /// Returns a single LoanPartDetails
  pub fn new(success: bool) -> ApiResultLoanPartDetails {
    ApiResultLoanPartDetails {
      payload: None,
      success: success,
      errors: None
    }
  }

  pub fn set_payload(&mut self, payload: crate::models::LoanPartDetails) {
    self.payload = Some(payload);
  }

  pub fn with_payload(mut self, payload: crate::models::LoanPartDetails) -> ApiResultLoanPartDetails {
    self.payload = Some(payload);
    self
  }

  pub fn payload(&self) -> Option<&crate::models::LoanPartDetails> {
    self.payload.as_ref()
  }

  pub fn reset_payload(&mut self) {
    self.payload = None;
  }

  pub fn set_success(&mut self, success: bool) {
    self.success = success;
  }

  pub fn with_success(mut self, success: bool) -> ApiResultLoanPartDetails {
    self.success = success;
    self
  }

  pub fn success(&self) -> &bool {
    &self.success
  }


  pub fn set_errors(&mut self, errors: Vec<crate::models::ApiError>) {
    self.errors = Some(errors);
  }

  pub fn with_errors(mut self, errors: Vec<crate::models::ApiError>) -> ApiResultLoanPartDetails {
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



