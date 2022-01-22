
/// BidRequest : Bids to make for the user.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct BidRequest {
  /// The bids to make.
  #[serde(rename = "Bids")]
  bids: Vec<crate::models::Bid>
}

impl BidRequest {
  /// Bids to make for the user.
  pub fn new(bids: Vec<crate::models::Bid>) -> BidRequest {
    BidRequest {
      bids: bids
    }
  }

  pub fn set_bids(&mut self, bids: Vec<crate::models::Bid>) {
    self.bids = bids;
  }

  pub fn with_bids(mut self, bids: Vec<crate::models::Bid>) -> BidRequest {
    self.bids = bids;
    self
  }

  pub fn bids(&self) -> &Vec<crate::models::Bid> {
    &self.bids
  }


}



