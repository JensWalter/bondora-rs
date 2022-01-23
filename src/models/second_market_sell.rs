/// SecondMarketSell : Sell loan from secondary market.
#[derive(Debug, Serialize, Deserialize)]
pub struct SecondMarketSell {
  /// LoanPart unique identifier
  #[serde(rename = "LoanPartId")]
  pub loan_part_id: Option<String>,
  /// Desired discount rate
  #[serde(rename = "DesiredDiscountRate")]
  pub desired_discount_rate: Option<i32>
}
