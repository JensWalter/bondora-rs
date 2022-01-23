#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResultSecondMarketItemSummary {
  /// The payload of the response. Type depends on the API request.
  #[serde(rename = "Payload")]
  pub payload: Option<crate::models::SecondMarketItemSummary>,
  /// Indicates if the request was successfull or not.  true if the request was handled successfully, false otherwise.
  #[serde(rename = "Success")]
  pub success: bool,
  /// Error(s) accociated with the API request.
  #[serde(rename = "Errors")]
  pub errors: Option<Vec<crate::models::ApiError>>
}
