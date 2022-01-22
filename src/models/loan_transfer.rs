

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct LoanTransfer {
  /// Payment date
  #[serde(rename = "Date")]
  date: Option<String>,
  /// Principal amount
  #[serde(rename = "PrincipalAmount")]
  principal_amount: Option<f64>,
  /// Interest amount
  #[serde(rename = "InterestAmount")]
  interest_amount: Option<f64>,
  /// Interest carried over amount
  #[serde(rename = "InterestAmountCarriedOver")]
  interest_amount_carried_over: Option<f64>,
  /// Penalty amount
  #[serde(rename = "PenaltyAmountCarriedOver")]
  penalty_amount_carried_over: Option<f64>,
  /// Total amount
  #[serde(rename = "TotalAmount")]
  total_amount: Option<f64>
}

impl LoanTransfer {
  pub fn new() -> LoanTransfer {
    LoanTransfer {
      date: None,
      principal_amount: None,
      interest_amount: None,
      interest_amount_carried_over: None,
      penalty_amount_carried_over: None,
      total_amount: None
    }
  }

  pub fn set_date(&mut self, date: String) {
    self.date = Some(date);
  }

  pub fn with_date(mut self, date: String) -> LoanTransfer {
    self.date = Some(date);
    self
  }

  pub fn date(&self) -> Option<&String> {
    self.date.as_ref()
  }

  pub fn reset_date(&mut self) {
    self.date = None;
  }

  pub fn set_principal_amount(&mut self, principal_amount: f64) {
    self.principal_amount = Some(principal_amount);
  }

  pub fn with_principal_amount(mut self, principal_amount: f64) -> LoanTransfer {
    self.principal_amount = Some(principal_amount);
    self
  }

  pub fn principal_amount(&self) -> Option<&f64> {
    self.principal_amount.as_ref()
  }

  pub fn reset_principal_amount(&mut self) {
    self.principal_amount = None;
  }

  pub fn set_interest_amount(&mut self, interest_amount: f64) {
    self.interest_amount = Some(interest_amount);
  }

  pub fn with_interest_amount(mut self, interest_amount: f64) -> LoanTransfer {
    self.interest_amount = Some(interest_amount);
    self
  }

  pub fn interest_amount(&self) -> Option<&f64> {
    self.interest_amount.as_ref()
  }

  pub fn reset_interest_amount(&mut self) {
    self.interest_amount = None;
  }

  pub fn set_interest_amount_carried_over(&mut self, interest_amount_carried_over: f64) {
    self.interest_amount_carried_over = Some(interest_amount_carried_over);
  }

  pub fn with_interest_amount_carried_over(mut self, interest_amount_carried_over: f64) -> LoanTransfer {
    self.interest_amount_carried_over = Some(interest_amount_carried_over);
    self
  }

  pub fn interest_amount_carried_over(&self) -> Option<&f64> {
    self.interest_amount_carried_over.as_ref()
  }

  pub fn reset_interest_amount_carried_over(&mut self) {
    self.interest_amount_carried_over = None;
  }

  pub fn set_penalty_amount_carried_over(&mut self, penalty_amount_carried_over: f64) {
    self.penalty_amount_carried_over = Some(penalty_amount_carried_over);
  }

  pub fn with_penalty_amount_carried_over(mut self, penalty_amount_carried_over: f64) -> LoanTransfer {
    self.penalty_amount_carried_over = Some(penalty_amount_carried_over);
    self
  }

  pub fn penalty_amount_carried_over(&self) -> Option<&f64> {
    self.penalty_amount_carried_over.as_ref()
  }

  pub fn reset_penalty_amount_carried_over(&mut self) {
    self.penalty_amount_carried_over = None;
  }

  pub fn set_total_amount(&mut self, total_amount: f64) {
    self.total_amount = Some(total_amount);
  }

  pub fn with_total_amount(mut self, total_amount: f64) -> LoanTransfer {
    self.total_amount = Some(total_amount);
    self
  }

  pub fn total_amount(&self) -> Option<&f64> {
    self.total_amount.as_ref()
  }

  pub fn reset_total_amount(&mut self) {
    self.total_amount = None;
  }

}



