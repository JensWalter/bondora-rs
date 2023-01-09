/// ApiError : API Error object. Describes the error that occured.
#[derive(Debug, Serialize, Deserialize)]
pub struct ApiError {
    /// Code of the error. For machine reading.
    #[serde(rename = "Code")]
    pub code: i32,
    /// The error message for human reading.
    #[serde(rename = "Message")]
    pub message: String,
    /// Error details, if any.   For example the non valid Field's name or the Id of the failed object.
    #[serde(rename = "Details")]
    pub details: Option<String>,
}
