
/// ApiResultLoanPartDetailsList : Returns a list of LoanPartDetails results

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResultLoanPartDetailsList {
  /// The payload of the response. Type depends on the API request.
  #[serde(rename = "Payload")]
  payload: Option<Vec<crate::models::LoanPartDetails>>,
  /// Indicates if the request was successfull or not.  true if the request was handled successfully, false otherwise.
  #[serde(rename = "Success")]
  success: bool,
  /// Error(s) accociated with the API request.
  #[serde(rename = "Errors")]
  errors: Option<Vec<crate::models::ApiError>>
}

impl ApiResultLoanPartDetailsList {
  /// Returns a list of LoanPartDetails results
  pub fn new(success: bool) -> ApiResultLoanPartDetailsList {
    ApiResultLoanPartDetailsList {
      payload: None,
      success: success,
      errors: None
    }
  }

  pub fn set_payload(&mut self, payload: Vec<crate::models::LoanPartDetails>) {
    self.payload = Some(payload);
  }

  pub fn with_payload(mut self, payload: Vec<crate::models::LoanPartDetails>) -> ApiResultLoanPartDetailsList {
    self.payload = Some(payload);
    self
  }

  pub fn payload(&self) -> Option<&Vec<crate::models::LoanPartDetails>> {
    self.payload.as_ref()
  }

  pub fn reset_payload(&mut self) {
    self.payload = None;
  }

  pub fn set_success(&mut self, success: bool) {
    self.success = success;
  }

  pub fn with_success(mut self, success: bool) -> ApiResultLoanPartDetailsList {
    self.success = success;
    self
  }

  pub fn success(&self) -> &bool {
    &self.success
  }


  pub fn set_errors(&mut self, errors: Vec<crate::models::ApiError>) {
    self.errors = Some(errors);
  }

  pub fn with_errors(mut self, errors: Vec<crate::models::ApiError>) -> ApiResultLoanPartDetailsList {
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



