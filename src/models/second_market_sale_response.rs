#[derive(Debug, Serialize, Deserialize)]
pub struct SecondMarketSaleResponse {
    /// Item unique identifier
    #[serde(rename = "Id")]
    pub id: Option<String>,
    /// Secondary market item unique identifier
    #[serde(rename = "LoanPartId")]
    pub loan_part_id: Option<String>,
    /// Desired discount rate
    #[serde(rename = "DesiredDiscountRate")]
    pub desired_discount_rate: Option<i32>,
}
