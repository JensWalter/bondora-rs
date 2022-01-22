

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ReportCreateRequest {
  #[serde(rename = "ReportType")]
  report_type: i32,
  #[serde(rename = "PeriodStart")]
  period_start: Option<String>,
  #[serde(rename = "PeriodEnd")]
  period_end: Option<String>
}

impl ReportCreateRequest {
  pub fn new(report_type: i32) -> ReportCreateRequest {
    ReportCreateRequest {
      report_type: report_type,
      period_start: None,
      period_end: None
    }
  }

  pub fn set_report_type(&mut self, report_type: i32) {
    self.report_type = report_type;
  }

  pub fn with_report_type(mut self, report_type: i32) -> ReportCreateRequest {
    self.report_type = report_type;
    self
  }

  pub fn report_type(&self) -> &i32 {
    &self.report_type
  }


  pub fn set_period_start(&mut self, period_start: String) {
    self.period_start = Some(period_start);
  }

  pub fn with_period_start(mut self, period_start: String) -> ReportCreateRequest {
    self.period_start = Some(period_start);
    self
  }

  pub fn period_start(&self) -> Option<&String> {
    self.period_start.as_ref()
  }

  pub fn reset_period_start(&mut self) {
    self.period_start = None;
  }

  pub fn set_period_end(&mut self, period_end: String) {
    self.period_end = Some(period_end);
  }

  pub fn with_period_end(mut self, period_end: String) -> ReportCreateRequest {
    self.period_end = Some(period_end);
    self
  }

  pub fn period_end(&self) -> Option<&String> {
    self.period_end.as_ref()
  }

  pub fn reset_period_end(&mut self) {
    self.period_end = None;
  }

}



