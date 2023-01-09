/// SecondMarketListingRequest : List of secondary market items to request info for.
#[derive(Debug, Serialize, Deserialize)]
pub struct SecondMarketListingRequest {
    /// Secondary market item ID's to list. Limited to 1000 items.
    #[serde(rename = "ItemIds")]
    pub item_ids: Option<Vec<String>>,
}
