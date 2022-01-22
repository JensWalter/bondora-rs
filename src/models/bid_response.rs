

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct BidResponse {
  /// Item unique identifier
  #[serde(rename = "Id")]
  id: Option<String>,
  /// Auction unique identifier
  #[serde(rename = "AuctionId")]
  auction_id: Option<String>,
  /// Amount to bid into Auction
  #[serde(rename = "Amount")]
  amount: Option<f64>,
  /// Not used. Will always be NULL.
  #[serde(rename = "MinAmount")]
  min_amount: Option<f64>
}

impl BidResponse {
  pub fn new() -> BidResponse {
    BidResponse {
      id: None,
      auction_id: None,
      amount: None,
      min_amount: None
    }
  }

  pub fn set_id(&mut self, id: String) {
    self.id = Some(id);
  }

  pub fn with_id(mut self, id: String) -> BidResponse {
    self.id = Some(id);
    self
  }

  pub fn id(&self) -> Option<&String> {
    self.id.as_ref()
  }

  pub fn reset_id(&mut self) {
    self.id = None;
  }

  pub fn set_auction_id(&mut self, auction_id: String) {
    self.auction_id = Some(auction_id);
  }

  pub fn with_auction_id(mut self, auction_id: String) -> BidResponse {
    self.auction_id = Some(auction_id);
    self
  }

  pub fn auction_id(&self) -> Option<&String> {
    self.auction_id.as_ref()
  }

  pub fn reset_auction_id(&mut self) {
    self.auction_id = None;
  }

  pub fn set_amount(&mut self, amount: f64) {
    self.amount = Some(amount);
  }

  pub fn with_amount(mut self, amount: f64) -> BidResponse {
    self.amount = Some(amount);
    self
  }

  pub fn amount(&self) -> Option<&f64> {
    self.amount.as_ref()
  }

  pub fn reset_amount(&mut self) {
    self.amount = None;
  }

  pub fn set_min_amount(&mut self, min_amount: f64) {
    self.min_amount = Some(min_amount);
  }

  pub fn with_min_amount(mut self, min_amount: f64) -> BidResponse {
    self.min_amount = Some(min_amount);
    self
  }

  pub fn min_amount(&self) -> Option<&f64> {
    self.min_amount.as_ref()
  }

  pub fn reset_min_amount(&mut self) {
    self.min_amount = None;
  }

}



