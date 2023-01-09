/// BidRequest : Bids to make for the user.
#[derive(Debug, Serialize, Deserialize)]
pub struct BidRequest {
    /// The bids to make.
    #[serde(rename = "Bids")]
    pub bids: Vec<crate::models::Bid>,
}
