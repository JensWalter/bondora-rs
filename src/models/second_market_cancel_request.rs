
/// SecondMarketCancelRequest : Secondary market sale request

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct SecondMarketCancelRequest {
  /// Secondary market item ids to cancel
  #[serde(rename = "ItemIds")]
  item_ids: Option<Vec<String>>
}

impl SecondMarketCancelRequest {
  /// Secondary market sale request
  pub fn new() -> SecondMarketCancelRequest {
    SecondMarketCancelRequest {
      item_ids: None
    }
  }

  pub fn set_item_ids(&mut self, item_ids: Vec<String>) {
    self.item_ids = Some(item_ids);
  }

  pub fn with_item_ids(mut self, item_ids: Vec<String>) -> SecondMarketCancelRequest {
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



