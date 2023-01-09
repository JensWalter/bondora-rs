/// ApiBidSummariesRequest : Request object for filtering api bids
#[derive(Debug, Serialize, Deserialize)]
pub struct ApiBidSummariesRequest {
    /// Bid status code
    #[serde(rename = "BidStatusCode")]
    pub bid_status_code: Option<i32>,
    /// Start date
    #[serde(rename = "StartDate")]
    pub start_date: Option<String>,
    /// End date
    #[serde(rename = "EndDate")]
    pub end_date: Option<String>,
    /// Max items in result, up to 20000
    #[serde(rename = "PageSize")]
    pub page_size: Option<i32>,
    /// Result page nr
    #[serde(rename = "PageNr")]
    pub page_nr: Option<i32>,
}
