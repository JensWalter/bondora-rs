

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct SecondMarketSaleResponse {
  /// Item unique identifier
  #[serde(rename = "Id")]
  id: Option<String>,
  /// Secondary market item unique identifier
  #[serde(rename = "LoanPartId")]
  loan_part_id: Option<String>,
  /// Desired discount rate
  #[serde(rename = "DesiredDiscountRate")]
  desired_discount_rate: Option<i32>
}

impl SecondMarketSaleResponse {
  pub fn new() -> SecondMarketSaleResponse {
    SecondMarketSaleResponse {
      id: None,
      loan_part_id: None,
      desired_discount_rate: None
    }
  }

  pub fn set_id(&mut self, id: String) {
    self.id = Some(id);
  }

  pub fn with_id(mut self, id: String) -> SecondMarketSaleResponse {
    self.id = Some(id);
    self
  }

  pub fn id(&self) -> Option<&String> {
    self.id.as_ref()
  }

  pub fn reset_id(&mut self) {
    self.id = None;
  }

  pub fn set_loan_part_id(&mut self, loan_part_id: String) {
    self.loan_part_id = Some(loan_part_id);
  }

  pub fn with_loan_part_id(mut self, loan_part_id: String) -> SecondMarketSaleResponse {
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

  pub fn with_desired_discount_rate(mut self, desired_discount_rate: i32) -> SecondMarketSaleResponse {
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



