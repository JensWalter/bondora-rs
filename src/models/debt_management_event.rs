
/// DebtManagementEvent : Information about the Loan's debt management stage event history

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct DebtManagementEvent {
  /// Date of event
  #[serde(rename = "CreatedOn")]
  created_on: Option<String>,
  /// Type of event
  #[serde(rename = "EventType")]
  event_type: Option<i32>,
  /// Type as a description, obsolete: use EventType
  #[serde(rename = "Description")]
  description: Option<String>
}

impl DebtManagementEvent {
  /// Information about the Loan's debt management stage event history
  pub fn new() -> DebtManagementEvent {
    DebtManagementEvent {
      created_on: None,
      event_type: None,
      description: None
    }
  }

  pub fn set_created_on(&mut self, created_on: String) {
    self.created_on = Some(created_on);
  }

  pub fn with_created_on(mut self, created_on: String) -> DebtManagementEvent {
    self.created_on = Some(created_on);
    self
  }

  pub fn created_on(&self) -> Option<&String> {
    self.created_on.as_ref()
  }

  pub fn reset_created_on(&mut self) {
    self.created_on = None;
  }

  pub fn set_event_type(&mut self, event_type: i32) {
    self.event_type = Some(event_type);
  }

  pub fn with_event_type(mut self, event_type: i32) -> DebtManagementEvent {
    self.event_type = Some(event_type);
    self
  }

  pub fn event_type(&self) -> Option<&i32> {
    self.event_type.as_ref()
  }

  pub fn reset_event_type(&mut self) {
    self.event_type = None;
  }

  pub fn set_description(&mut self, description: String) {
    self.description = Some(description);
  }

  pub fn with_description(mut self, description: String) -> DebtManagementEvent {
    self.description = Some(description);
    self
  }

  pub fn description(&self) -> Option<&String> {
    self.description.as_ref()
  }

  pub fn reset_description(&mut self) {
    self.description = None;
  }

}



