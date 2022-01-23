#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResultPublicDataset {
  /// Requested Max items in result
  #[serde(rename = "PageSize")]
  pub page_size: Option<i32>,
  /// Requested page nr
  #[serde(rename = "PageNr")]
  pub page_nr: Option<i32>,
  /// Total number of items found
  #[serde(rename = "TotalCount")]
  pub total_count: i32,
  /// Number of items returned
  #[serde(rename = "Count")]
  pub count: i32,
  /// The payload of the response. Type depends on the API request.
  #[serde(rename = "Payload")]
  pub payload: Option<Vec<crate::models::PublicDatasetItem>>,
  /// Indicates if the request was successfull or not.  true if the request was handled successfully, false otherwise.
  #[serde(rename = "Success")]
  pub success: bool,
  /// Error(s) accociated with the API request.
  #[serde(rename = "Errors")]
  pub errors: Option<Vec<crate::models::ApiError>>
}
