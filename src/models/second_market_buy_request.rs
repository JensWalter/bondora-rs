/// SecondMarketBuyRequest : Buy loans from secondary market for the user.
#[derive(Debug, Serialize, Deserialize)]
pub struct SecondMarketBuyRequest {
  /// Secondary market item ids to buy
  #[serde(rename = "ItemIds")]
  pub item_ids: Option<Vec<String>>
}
