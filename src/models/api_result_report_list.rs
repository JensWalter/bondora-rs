#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResultReportList {
    /// Number of items returned
    #[serde(rename = "Count")]
    pub count: i32,
    /// The payload of the response. Type depends on the API request.
    #[serde(rename = "Payload")]
    pub payload: Option<Vec<crate::models::ReportItem>>,
    /// Indicates if the request was successfull or not.  true if the request was handled successfully, false otherwise.
    #[serde(rename = "Success")]
    pub success: bool,
    /// Error(s) accociated with the API request.
    #[serde(rename = "Errors")]
    pub errors: Option<Vec<crate::models::ApiError>>,
}
