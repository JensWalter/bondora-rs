/// Report : Report data with data rows
#[derive(Debug, Serialize, Deserialize)]
pub struct Report {
  /// <para>List of Report items. Item type depends on {Bondora.Core.Enums.ReportType} value:</para>  <list type=\"bullet\">    <item>      <term>SecondMarketArchive</term>      <description>the type is {Sobralaen.Api.Models.SecondMarketArchiveReportLine}</description>    </item>    <item>      <term>AccountStatement</term>      <description>the type is {Sobralaen.Api.Models.AccountStatementReportLine}</description>    </item>    <item>      <term>Repayments</term>      <description>the type is {Sobralaen.Api.Models.RepaymentsReportLine}</description>    </item>    <item>      <term>Investments</term>      <description>the type is {Sobralaen.Api.Models.InvestmentsListReportLine}</description>    </item>    <item>      <term>PlannedFutureCashflows</term>      <description>the type is {Sobralaen.Api.Models.FutureCashflowsReportLine}</description>    </item>    <item>      <term>InvestmentsV2</term>      <description>the type is {Sobralaen.Api.Models.InvestmentsListReportLineV2}</description>    </item>  </list>
  #[serde(rename = "Result")]
  pub result: Option<Vec<serde_json::Value>>,
  /// Reports unique identifier
  #[serde(rename = "ReportId")]
  pub report_id: Option<String>,
  /// Report created date
  #[serde(rename = "CreatedOn")]
  pub created_on: Option<String>,
  /// Report generated date
  #[serde(rename = "GeneratedOn")]
  pub generated_on: Option<String>,
  /// Report period end date
  #[serde(rename = "PeriodStart")]
  pub period_start: Option<String>,
  /// Report period start date
  #[serde(rename = "PeriodEnd")]
  pub period_end: Option<String>,
  /// Report's type
  #[serde(rename = "ReportType")]
  pub report_type: Option<i32>
}
