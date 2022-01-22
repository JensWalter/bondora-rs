

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct SecondMarketItemSummary {
  /// Item unique identifier
  #[serde(rename = "Id")]
  id: Option<String>,
  /// Number
  #[serde(rename = "Number")]
  number: Option<i32>,
  /// Item start date
  #[serde(rename = "StartDate")]
  start_date: Option<String>,
  /// Planned close date
  #[serde(rename = "PlannedCloseDate")]
  planned_close_date: Option<String>,
  /// Actual close date
  #[serde(rename = "ActualCloseDate")]
  actual_close_date: Option<String>,
  /// User cancelled on
  #[serde(rename = "UserCancelledOn")]
  user_cancelled_on: Option<String>,
  /// LoanPart being sold
  #[serde(rename = "LoanPart_id")]
  loan_part_id: Option<String>,
  /// Discount rate percent
  #[serde(rename = "DesiredDiscountRate")]
  desired_discount_rate: Option<f64>,
  /// Discount rate as fraction (0.0 - 1.0)
  #[serde(rename = "DesiredDiscountRateDecimalFraction")]
  desired_discount_rate_decimal_fraction: Option<f64>,
  /// Current status code  <para>0 Created</para><para>1 Open in marketplace</para><para>2 Successfully sold</para><para>3 Failed</para>
  #[serde(rename = "StatusCode")]
  status_code: Option<i32>
}

impl SecondMarketItemSummary {
  pub fn new() -> SecondMarketItemSummary {
    SecondMarketItemSummary {
      id: None,
      number: None,
      start_date: None,
      planned_close_date: None,
      actual_close_date: None,
      user_cancelled_on: None,
      loan_part_id: None,
      desired_discount_rate: None,
      desired_discount_rate_decimal_fraction: None,
      status_code: None
    }
  }

  pub fn set_id(&mut self, id: String) {
    self.id = Some(id);
  }

  pub fn with_id(mut self, id: String) -> SecondMarketItemSummary {
    self.id = Some(id);
    self
  }

  pub fn id(&self) -> Option<&String> {
    self.id.as_ref()
  }

  pub fn reset_id(&mut self) {
    self.id = None;
  }

  pub fn set_number(&mut self, number: i32) {
    self.number = Some(number);
  }

  pub fn with_number(mut self, number: i32) -> SecondMarketItemSummary {
    self.number = Some(number);
    self
  }

  pub fn number(&self) -> Option<&i32> {
    self.number.as_ref()
  }

  pub fn reset_number(&mut self) {
    self.number = None;
  }

  pub fn set_start_date(&mut self, start_date: String) {
    self.start_date = Some(start_date);
  }

  pub fn with_start_date(mut self, start_date: String) -> SecondMarketItemSummary {
    self.start_date = Some(start_date);
    self
  }

  pub fn start_date(&self) -> Option<&String> {
    self.start_date.as_ref()
  }

  pub fn reset_start_date(&mut self) {
    self.start_date = None;
  }

  pub fn set_planned_close_date(&mut self, planned_close_date: String) {
    self.planned_close_date = Some(planned_close_date);
  }

  pub fn with_planned_close_date(mut self, planned_close_date: String) -> SecondMarketItemSummary {
    self.planned_close_date = Some(planned_close_date);
    self
  }

  pub fn planned_close_date(&self) -> Option<&String> {
    self.planned_close_date.as_ref()
  }

  pub fn reset_planned_close_date(&mut self) {
    self.planned_close_date = None;
  }

  pub fn set_actual_close_date(&mut self, actual_close_date: String) {
    self.actual_close_date = Some(actual_close_date);
  }

  pub fn with_actual_close_date(mut self, actual_close_date: String) -> SecondMarketItemSummary {
    self.actual_close_date = Some(actual_close_date);
    self
  }

  pub fn actual_close_date(&self) -> Option<&String> {
    self.actual_close_date.as_ref()
  }

  pub fn reset_actual_close_date(&mut self) {
    self.actual_close_date = None;
  }

  pub fn set_user_cancelled_on(&mut self, user_cancelled_on: String) {
    self.user_cancelled_on = Some(user_cancelled_on);
  }

  pub fn with_user_cancelled_on(mut self, user_cancelled_on: String) -> SecondMarketItemSummary {
    self.user_cancelled_on = Some(user_cancelled_on);
    self
  }

  pub fn user_cancelled_on(&self) -> Option<&String> {
    self.user_cancelled_on.as_ref()
  }

  pub fn reset_user_cancelled_on(&mut self) {
    self.user_cancelled_on = None;
  }

  pub fn set_loan_part_id(&mut self, loan_part_id: String) {
    self.loan_part_id = Some(loan_part_id);
  }

  pub fn with_loan_part_id(mut self, loan_part_id: String) -> SecondMarketItemSummary {
    self.loan_part_id = Some(loan_part_id);
    self
  }

  pub fn loan_part_id(&self) -> Option<&String> {
    self.loan_part_id.as_ref()
  }

  pub fn reset_loan_part_id(&mut self) {
    self.loan_part_id = None;
  }

  pub fn set_desired_discount_rate(&mut self, desired_discount_rate: f64) {
    self.desired_discount_rate = Some(desired_discount_rate);
  }

  pub fn with_desired_discount_rate(mut self, desired_discount_rate: f64) -> SecondMarketItemSummary {
    self.desired_discount_rate = Some(desired_discount_rate);
    self
  }

  pub fn desired_discount_rate(&self) -> Option<&f64> {
    self.desired_discount_rate.as_ref()
  }

  pub fn reset_desired_discount_rate(&mut self) {
    self.desired_discount_rate = None;
  }

  pub fn set_desired_discount_rate_decimal_fraction(&mut self, desired_discount_rate_decimal_fraction: f64) {
    self.desired_discount_rate_decimal_fraction = Some(desired_discount_rate_decimal_fraction);
  }

  pub fn with_desired_discount_rate_decimal_fraction(mut self, desired_discount_rate_decimal_fraction: f64) -> SecondMarketItemSummary {
    self.desired_discount_rate_decimal_fraction = Some(desired_discount_rate_decimal_fraction);
    self
  }

  pub fn desired_discount_rate_decimal_fraction(&self) -> Option<&f64> {
    self.desired_discount_rate_decimal_fraction.as_ref()
  }

  pub fn reset_desired_discount_rate_decimal_fraction(&mut self) {
    self.desired_discount_rate_decimal_fraction = None;
  }

  pub fn set_status_code(&mut self, status_code: i32) {
    self.status_code = Some(status_code);
  }

  pub fn with_status_code(mut self, status_code: i32) -> SecondMarketItemSummary {
    self.status_code = Some(status_code);
    self
  }

  pub fn status_code(&self) -> Option<&i32> {
    self.status_code.as_ref()
  }

  pub fn reset_status_code(&mut self) {
    self.status_code = None;
  }

}



