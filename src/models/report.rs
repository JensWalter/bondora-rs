
/// Report : Report data with data rows

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct Report {
  /// <para>List of Report items. Item type depends on {Bondora.Core.Enums.ReportType} value:</para>  <list type=\"bullet\">    <item>      <term>SecondMarketArchive</term>      <description>the type is {Sobralaen.Api.Models.SecondMarketArchiveReportLine}</description>    </item>    <item>      <term>AccountStatement</term>      <description>the type is {Sobralaen.Api.Models.AccountStatementReportLine}</description>    </item>    <item>      <term>Repayments</term>      <description>the type is {Sobralaen.Api.Models.RepaymentsReportLine}</description>    </item>    <item>      <term>Investments</term>      <description>the type is {Sobralaen.Api.Models.InvestmentsListReportLine}</description>    </item>    <item>      <term>PlannedFutureCashflows</term>      <description>the type is {Sobralaen.Api.Models.FutureCashflowsReportLine}</description>    </item>    <item>      <term>InvestmentsV2</term>      <description>the type is {Sobralaen.Api.Models.InvestmentsListReportLineV2}</description>    </item>  </list>
  #[serde(rename = "Result")]
  result: Option<Vec<Value>>,
  /// Reports unique identifier
  #[serde(rename = "ReportId")]
  report_id: Option<String>,
  /// Report created date
  #[serde(rename = "CreatedOn")]
  created_on: Option<String>,
  /// Report generated date
  #[serde(rename = "GeneratedOn")]
  generated_on: Option<String>,
  /// Report period end date
  #[serde(rename = "PeriodStart")]
  period_start: Option<String>,
  /// Report period start date
  #[serde(rename = "PeriodEnd")]
  period_end: Option<String>,
  /// Report's type
  #[serde(rename = "ReportType")]
  report_type: Option<i32>
}

impl Report {
  /// Report data with data rows
  pub fn new() -> Report {
    Report {
      result: None,
      report_id: None,
      created_on: None,
      generated_on: None,
      period_start: None,
      period_end: None,
      report_type: None
    }
  }

  pub fn set_result(&mut self, result: Vec<Value>) {
    self.result = Some(result);
  }

  pub fn with_result(mut self, result: Vec<Value>) -> Report {
    self.result = Some(result);
    self
  }

  pub fn result(&self) -> Option<&Vec<Value>> {
    self.result.as_ref()
  }

  pub fn reset_result(&mut self) {
    self.result = None;
  }

  pub fn set_report_id(&mut self, report_id: String) {
    self.report_id = Some(report_id);
  }

  pub fn with_report_id(mut self, report_id: String) -> Report {
    self.report_id = Some(report_id);
    self
  }

  pub fn report_id(&self) -> Option<&String> {
    self.report_id.as_ref()
  }

  pub fn reset_report_id(&mut self) {
    self.report_id = None;
  }

  pub fn set_created_on(&mut self, created_on: String) {
    self.created_on = Some(created_on);
  }

  pub fn with_created_on(mut self, created_on: String) -> Report {
    self.created_on = Some(created_on);
    self
  }

  pub fn created_on(&self) -> Option<&String> {
    self.created_on.as_ref()
  }

  pub fn reset_created_on(&mut self) {
    self.created_on = None;
  }

  pub fn set_generated_on(&mut self, generated_on: String) {
    self.generated_on = Some(generated_on);
  }

  pub fn with_generated_on(mut self, generated_on: String) -> Report {
    self.generated_on = Some(generated_on);
    self
  }

  pub fn generated_on(&self) -> Option<&String> {
    self.generated_on.as_ref()
  }

  pub fn reset_generated_on(&mut self) {
    self.generated_on = None;
  }

  pub fn set_period_start(&mut self, period_start: String) {
    self.period_start = Some(period_start);
  }

  pub fn with_period_start(mut self, period_start: String) -> Report {
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

  pub fn with_period_end(mut self, period_end: String) -> Report {
    self.period_end = Some(period_end);
    self
  }

  pub fn period_end(&self) -> Option<&String> {
    self.period_end.as_ref()
  }

  pub fn reset_period_end(&mut self) {
    self.period_end = None;
  }

  pub fn set_report_type(&mut self, report_type: i32) {
    self.report_type = Some(report_type);
  }

  pub fn with_report_type(mut self, report_type: i32) -> Report {
    self.report_type = Some(report_type);
    self
  }

  pub fn report_type(&self) -> Option<&i32> {
    self.report_type.as_ref()
  }

  pub fn reset_report_type(&mut self) {
    self.report_type = None;
  }

}



