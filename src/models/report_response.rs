#[derive(Debug, Serialize, Deserialize)]
pub struct ReportResponse {
  /// Item unique identifier
  #[serde(rename = "ReportId")]
  pub report_id: Option<String>
}
