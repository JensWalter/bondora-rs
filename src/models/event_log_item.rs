
/// EventLogItem : EventLog list item's information

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct EventLogItem {
  /// Event date
  #[serde(rename = "EventDate")]
  event_date: Option<String>,
  /// Event type
  #[serde(rename = "EventType")]
  event_type: Option<i32>,
  /// IP address
  #[serde(rename = "IpAddress")]
  ip_address: Option<String>,
  /// Request data JSON format
  #[serde(rename = "Data")]
  data: Option<String>
}

impl EventLogItem {
  /// EventLog list item's information
  pub fn new() -> EventLogItem {
    EventLogItem {
      event_date: None,
      event_type: None,
      ip_address: None,
      data: None
    }
  }

  pub fn set_event_date(&mut self, event_date: String) {
    self.event_date = Some(event_date);
  }

  pub fn with_event_date(mut self, event_date: String) -> EventLogItem {
    self.event_date = Some(event_date);
    self
  }

  pub fn event_date(&self) -> Option<&String> {
    self.event_date.as_ref()
  }

  pub fn reset_event_date(&mut self) {
    self.event_date = None;
  }

  pub fn set_event_type(&mut self, event_type: i32) {
    self.event_type = Some(event_type);
  }

  pub fn with_event_type(mut self, event_type: i32) -> EventLogItem {
    self.event_type = Some(event_type);
    self
  }

  pub fn event_type(&self) -> Option<&i32> {
    self.event_type.as_ref()
  }

  pub fn reset_event_type(&mut self) {
    self.event_type = None;
  }

  pub fn set_ip_address(&mut self, ip_address: String) {
    self.ip_address = Some(ip_address);
  }

  pub fn with_ip_address(mut self, ip_address: String) -> EventLogItem {
    self.ip_address = Some(ip_address);
    self
  }

  pub fn ip_address(&self) -> Option<&String> {
    self.ip_address.as_ref()
  }

  pub fn reset_ip_address(&mut self) {
    self.ip_address = None;
  }

  pub fn set_data(&mut self, data: String) {
    self.data = Some(data);
  }

  pub fn with_data(mut self, data: String) -> EventLogItem {
    self.data = Some(data);
    self
  }

  pub fn data(&self) -> Option<&String> {
    self.data.as_ref()
  }

  pub fn reset_data(&mut self) {
    self.data = None;
  }

}



