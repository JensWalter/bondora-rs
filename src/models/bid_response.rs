#[derive(Debug, Serialize, Deserialize)]
pub struct BidResponse {
  /// Item unique identifier
  #[serde(rename = "Id")]
  pub id: Option<String>,
  /// Auction unique identifier
  #[serde(rename = "AuctionId")]
  pub auction_id: Option<String>,
  /// Amount to bid into Auction
  #[serde(rename = "Amount")]
  pub amount: Option<f64>,
  /// Not used. Will always be NULL.
  #[serde(rename = "MinAmount")]
  pub min_amount: Option<f64>
}
