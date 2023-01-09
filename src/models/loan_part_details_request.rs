/// LoanPartDetailsRequest : Container for LoanPartDetails item ID's.
#[derive(Debug, Serialize, Deserialize)]
pub struct LoanPartDetailsRequest {
    /// LoanPart ID's to list. Limited to 1000 items.
    #[serde(rename = "ItemIds")]
    pub item_ids: Option<Vec<String>>,
}
