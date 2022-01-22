/// ApiError : API Error object. Describes the error that occured.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiError {
  /// Code of the error. For machine reading.
  #[serde(rename = "Code")]
  code: i32,
  /// The error message for human reading.
  #[serde(rename = "Message")]
  message: String,
  /// Error details, if any.   For example the non valid Field's name or the Id of the failed object.
  #[serde(rename = "Details")]
  details: Option<String>
}

impl ApiError {
  /// API Error object. Describes the error that occured.
  pub fn new(code: i32, message: String) -> ApiError {
    ApiError {
      code: code,
      message: message,
      details: None
    }
  }

  pub fn set_code(&mut self, code: i32) {
    self.code = code;
  }

  pub fn with_code(mut self, code: i32) -> ApiError {
    self.code = code;
    self
  }

  pub fn code(&self) -> &i32 {
    &self.code
  }


  pub fn set_message(&mut self, message: String) {
    self.message = message;
  }

  pub fn with_message(mut self, message: String) -> ApiError {
    self.message = message;
    self
  }

  pub fn message(&self) -> &String {
    &self.message
  }


  pub fn set_details(&mut self, details: String) {
    self.details = Some(details);
  }

  pub fn with_details(mut self, details: String) -> ApiError {
    self.details = Some(details);
    self
  }

  pub fn details(&self) -> Option<&String> {
    self.details.as_ref()
  }

  pub fn reset_details(&mut self) {
    self.details = None;
  }

}



