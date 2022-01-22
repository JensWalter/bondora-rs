
/// SecondMarketListingRequest : List of secondary market items to request info for.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct SecondMarketListingRequest {
  /// Secondary market item ID's to list. Limited to 1000 items.
  #[serde(rename = "ItemIds")]
  item_ids: Option<Vec<String>>
}

impl SecondMarketListingRequest {
  /// List of secondary market items to request info for.
  pub fn new() -> SecondMarketListingRequest {
    SecondMarketListingRequest {
      item_ids: None
    }
  }

  pub fn set_item_ids(&mut self, item_ids: Vec<String>) {
    self.item_ids = Some(item_ids);
  }

  pub fn with_item_ids(mut self, item_ids: Vec<String>) -> SecondMarketListingRequest {
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



