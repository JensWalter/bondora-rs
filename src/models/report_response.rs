

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ReportResponse {
  /// Item unique identifier
  #[serde(rename = "ReportId")]
  report_id: Option<String>
}

impl ReportResponse {
  pub fn new() -> ReportResponse {
    ReportResponse {
      report_id: None
    }
  }

  pub fn set_report_id(&mut self, report_id: String) {
    self.report_id = Some(report_id);
  }

  pub fn with_report_id(mut self, report_id: String) -> ReportResponse {
    self.report_id = Some(report_id);
    self
  }

  pub fn report_id(&self) -> Option<&String> {
    self.report_id.as_ref()
  }

  pub fn reset_report_id(&mut self) {
    self.report_id = None;
  }

}



