#[derive(Debug, Serialize, Deserialize)]
pub struct ReportCreateRequest {
    #[serde(rename = "ReportType")]
    pub report_type: i32,
    #[serde(rename = "PeriodStart")]
    pub period_start: Option<String>,
    #[serde(rename = "PeriodEnd")]
    pub period_end: Option<String>,
}
