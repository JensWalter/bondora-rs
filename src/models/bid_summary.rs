/// BidSummary : Bid's information
#[derive(Debug, Serialize, Deserialize)]
pub struct BidSummary {
    /// Bid unique identifier
    #[serde(rename = "Id")]
    pub id: Option<String>,
    /// Id of auction to bid into
    #[serde(rename = "AuctionId")]
    pub auction_id: Option<String>,
    /// Amount that was requested to bid
    #[serde(rename = "RequestedBidAmount")]
    pub requested_bid_amount: Option<f64>,
    /// Amount that is bidded
    #[serde(rename = "ActualBidAmount")]
    pub actual_bid_amount: Option<f64>,
    /// Minimum amount that was specified for auction
    #[serde(rename = "RequestedBidMinimumLimit")]
    pub requested_bid_minimum_limit: Option<f64>,
    /// When bid was requested
    #[serde(rename = "BidRequestedDate")]
    pub bid_requested_date: Option<String>,
    /// When bid was processed
    #[serde(rename = "BidProcessedDate")]
    pub bid_processed_date: Option<String>,
    /// Is request currently processed
    #[serde(rename = "IsRequestBeingProcessed")]
    pub is_request_being_processed: Option<bool>,
    /// Status of bid  <para>0 Pending</para><para>1 Open</para><para>2 Successful</para><para>3 Failed</para><para>4 Cancelled</para><para>5 Accepted</para>
    #[serde(rename = "StatusCode")]
    pub status_code: Option<i32>,
    /// Why bid failed
    #[serde(rename = "FailureReason")]
    pub failure_reason: Option<i32>,
}
