/// Bid : Bid to make into Auction.
#[derive(Debug, Serialize, Deserialize)]
pub struct Bid {
    /// The Auction's ID to bid to.
    #[serde(rename = "AuctionId")]
    pub auction_id: String,
    /// Amount to bid into Auction
    #[serde(rename = "Amount")]
    pub amount: f64,
    /// Not used. Value will be ignored.
    #[serde(rename = "MinAmount")]
    pub min_amount: Option<f64>,
}
