
/// Bid : Bid to make into Auction.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct Bid {
  /// The Auction's ID to bid to.
  #[serde(rename = "AuctionId")]
  auction_id: String,
  /// Amount to bid into Auction
  #[serde(rename = "Amount")]
  amount: f64,
  /// Not used. Value will be ignored.
  #[serde(rename = "MinAmount")]
  min_amount: Option<f64>
}

impl Bid {
  /// Bid to make into Auction.
  pub fn new(auction_id: String, amount: f64) -> Bid {
    Bid {
      auction_id: auction_id,
      amount: amount,
      min_amount: None
    }
  }

  pub fn set_auction_id(&mut self, auction_id: String) {
    self.auction_id = auction_id;
  }

  pub fn with_auction_id(mut self, auction_id: String) -> Bid {
    self.auction_id = auction_id;
    self
  }

  pub fn auction_id(&self) -> &String {
    &self.auction_id
  }


  pub fn set_amount(&mut self, amount: f64) {
    self.amount = amount;
  }

  pub fn with_amount(mut self, amount: f64) -> Bid {
    self.amount = amount;
    self
  }

  pub fn amount(&self) -> &f64 {
    &self.amount
  }


  pub fn set_min_amount(&mut self, min_amount: f64) {
    self.min_amount = Some(min_amount);
  }

  pub fn with_min_amount(mut self, min_amount: f64) -> Bid {
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



