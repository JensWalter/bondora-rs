/// SecondMarketCancelRequest : Secondary market sale request
#[derive(Debug, Serialize, Deserialize)]
pub struct SecondMarketCancelRequest {
    /// Secondary market item ids to cancel
    #[serde(rename = "ItemIds")]
    pub item_ids: Option<Vec<String>>,
}
