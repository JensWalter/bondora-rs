/// ReportItem : Generec Report data
#[derive(Debug, Serialize, Deserialize)]
pub struct ReportItem {
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
    pub report_type: Option<i32>,
}
