
/// SecondMarketBuyRequest : Buy loans from secondary market for the user.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct SecondMarketBuyRequest {
  /// Secondary market item ids to buy
  #[serde(rename = "ItemIds")]
  item_ids: Option<Vec<String>>
}

impl SecondMarketBuyRequest {
  /// Buy loans from secondary market for the user.
  pub fn new() -> SecondMarketBuyRequest {
    SecondMarketBuyRequest {
      item_ids: None
    }
  }

  pub fn set_item_ids(&mut self, item_ids: Vec<String>) {
    self.item_ids = Some(item_ids);
  }

  pub fn with_item_ids(mut self, item_ids: Vec<String>) -> SecondMarketBuyRequest {
    self.item_ids = Some(item_ids);
    self
  }

  pub fn item_ids(&self) -> Option<&Vec<String>> {
    self.item_ids.as_ref()
  }

  pub fn reset_item_ids(&mut self) {
    self.item_ids = None;
  }

}



