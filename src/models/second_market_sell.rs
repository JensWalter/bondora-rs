
/// SecondMarketSell : Sell loan from secondary market.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct SecondMarketSell {
  /// LoanPart unique identifier
  #[serde(rename = "LoanPartId")]
  loan_part_id: Option<String>,
  /// Desired discount rate
  #[serde(rename = "DesiredDiscountRate")]
  desired_discount_rate: Option<i32>
}

impl SecondMarketSell {
  /// Sell loan from secondary market.
  pub fn new() -> SecondMarketSell {
    SecondMarketSell {
      loan_part_id: None,
      desired_discount_rate: None
    }
  }

  pub fn set_loan_part_id(&mut self, loan_part_id: String) {
    self.loan_part_id = Some(loan_part_id);
  }

  pub fn with_loan_part_id(mut self, loan_part_id: String) -> SecondMarketSell {
    self.loan_part_id = Some(loan_part_id);
    self
  }

  pub fn loan_part_id(&self) -> Option<&String> {
    self.loan_part_id.as_ref()
  }

  pub fn reset_loan_part_id(&mut self) {
    self.loan_part_id = None;
  }

  pub fn set_desired_discount_rate(&mut self, desired_discount_rate: i32) {
    self.desired_discount_rate = Some(desired_discount_rate);
  }

  pub fn with_desired_discount_rate(mut self, desired_discount_rate: i32) -> SecondMarketSell {
    self.desired_discount_rate = Some(desired_discount_rate);
    self
  }

  pub fn desired_discount_rate(&self) -> Option<&i32> {
    self.desired_discount_rate.as_ref()
  }

  pub fn reset_desired_discount_rate(&mut self) {
    self.desired_discount_rate = None;
  }

}



