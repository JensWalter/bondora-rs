
/// EventLogRequest : Request object for filtering event log

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct EventLogRequest {
  /// Start datetime
  #[serde(rename = "EventDateFrom")]
  event_date_from: Option<String>,
  /// end datetime
  #[serde(rename = "EventDateTo")]
  event_date_to: Option<String>,
  /// Event type
  #[serde(rename = "EventType")]
  event_type: Option<i32>,
  /// IP address
  #[serde(rename = "IpAddress")]
  ip_address: Option<String>,
  /// Max items in result, up to 20000
  #[serde(rename = "PageSize")]
  page_size: Option<i32>,
  /// Result page nr
  #[serde(rename = "PageNr")]
  page_nr: Option<i32>
}

impl EventLogRequest {
  /// Request object for filtering event log
  pub fn new() -> EventLogRequest {
    EventLogRequest {
      event_date_from: None,
      event_date_to: None,
      event_type: None,
      ip_address: None,
      page_size: None,
      page_nr: None
    }
  }

  pub fn set_event_date_from(&mut self, event_date_from: String) {
    self.event_date_from = Some(event_date_from);
  }

  pub fn with_event_date_from(mut self, event_date_from: String) -> EventLogRequest {
    self.event_date_from = Some(event_date_from);
    self
  }

  pub fn event_date_from(&self) -> Option<&String> {
    self.event_date_from.as_ref()
  }

  pub fn reset_event_date_from(&mut self) {
    self.event_date_from = None;
  }

  pub fn set_event_date_to(&mut self, event_date_to: String) {
    self.event_date_to = Some(event_date_to);
  }

  pub fn with_event_date_to(mut self, event_date_to: String) -> EventLogRequest {
    self.event_date_to = Some(event_date_to);
    self
  }

  pub fn event_date_to(&self) -> Option<&String> {
    self.event_date_to.as_ref()
  }

  pub fn reset_event_date_to(&mut self) {
    self.event_date_to = None;
  }

  pub fn set_event_type(&mut self, event_type: i32) {
    self.event_type = Some(event_type);
  }

  pub fn with_event_type(mut self, event_type: i32) -> EventLogRequest {
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

  pub fn with_ip_address(mut self, ip_address: String) -> EventLogRequest {
    self.ip_address = Some(ip_address);
    self
  }

  pub fn ip_address(&self) -> Option<&String> {
    self.ip_address.as_ref()
  }

  pub fn reset_ip_address(&mut self) {
    self.ip_address = None;
  }

  pub fn set_page_size(&mut self, page_size: i32) {
    self.page_size = Some(page_size);
  }

  pub fn with_page_size(mut self, page_size: i32) -> EventLogRequest {
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

  pub fn with_page_nr(mut self, page_nr: i32) -> EventLogRequest {
    self.page_nr = Some(page_nr);
    self
  }

  pub fn page_nr(&self) -> Option<&i32> {
    self.page_nr.as_ref()
  }

  pub fn reset_page_nr(&mut self) {
    self.page_nr = None;
  }

}



