
/// ScheduledPayment : Scheduled payment item information

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ScheduledPayment {
  /// Scheduled payment date
  #[serde(rename = "ScheduledDate")]
  scheduled_date: Option<String>,
  /// Scheduled principal amount
  #[serde(rename = "PrincipalAmount")]
  principal_amount: Option<f64>,
  /// Scheduled principal amount remaining after payment
  #[serde(rename = "PrincipalAmountLeft")]
  principal_amount_left: Option<f64>,
  /// Scheduled interest amount
  #[serde(rename = "InterestAmount")]
  interest_amount: Option<f64>,
  /// Interest amount carried over from rescheduling
  #[serde(rename = "IntrestAmountCarriedOver")]
  intrest_amount_carried_over: Option<f64>,
  /// Penalty amount carried over from rescheduling
  #[serde(rename = "PenaltyAmountCarriedOver")]
  penalty_amount_carried_over: Option<f64>,
  /// Total payment amount
  #[serde(rename = "TotalAmount")]
  total_amount: Option<f64>
}

impl ScheduledPayment {
  /// Scheduled payment item information
  pub fn new() -> ScheduledPayment {
    ScheduledPayment {
      scheduled_date: None,
      principal_amount: None,
      principal_amount_left: None,
      interest_amount: None,
      intrest_amount_carried_over: None,
      penalty_amount_carried_over: None,
      total_amount: None
    }
  }

  pub fn set_scheduled_date(&mut self, scheduled_date: String) {
    self.scheduled_date = Some(scheduled_date);
  }

  pub fn with_scheduled_date(mut self, scheduled_date: String) -> ScheduledPayment {
    self.scheduled_date = Some(scheduled_date);
    self
  }

  pub fn scheduled_date(&self) -> Option<&String> {
    self.scheduled_date.as_ref()
  }

  pub fn reset_scheduled_date(&mut self) {
    self.scheduled_date = None;
  }

  pub fn set_principal_amount(&mut self, principal_amount: f64) {
    self.principal_amount = Some(principal_amount);
  }

  pub fn with_principal_amount(mut self, principal_amount: f64) -> ScheduledPayment {
    self.principal_amount = Some(principal_amount);
    self
  }

  pub fn principal_amount(&self) -> Option<&f64> {
    self.principal_amount.as_ref()
  }

  pub fn reset_principal_amount(&mut self) {
    self.principal_amount = None;
  }

  pub fn set_principal_amount_left(&mut self, principal_amount_left: f64) {
    self.principal_amount_left = Some(principal_amount_left);
  }

  pub fn with_principal_amount_left(mut self, principal_amount_left: f64) -> ScheduledPayment {
    self.principal_amount_left = Some(principal_amount_left);
    self
  }

  pub fn principal_amount_left(&self) -> Option<&f64> {
    self.principal_amount_left.as_ref()
  }

  pub fn reset_principal_amount_left(&mut self) {
    self.principal_amount_left = None;
  }

  pub fn set_interest_amount(&mut self, interest_amount: f64) {
    self.interest_amount = Some(interest_amount);
  }

  pub fn with_interest_amount(mut self, interest_amount: f64) -> ScheduledPayment {
    self.interest_amount = Some(interest_amount);
    self
  }

  pub fn interest_amount(&self) -> Option<&f64> {
    self.interest_amount.as_ref()
  }

  pub fn reset_interest_amount(&mut self) {
    self.interest_amount = None;
  }

  pub fn set_intrest_amount_carried_over(&mut self, intrest_amount_carried_over: f64) {
    self.intrest_amount_carried_over = Some(intrest_amount_carried_over);
  }

  pub fn with_intrest_amount_carried_over(mut self, intrest_amount_carried_over: f64) -> ScheduledPayment {
    self.intrest_amount_carried_over = Some(intrest_amount_carried_over);
    self
  }

  pub fn intrest_amount_carried_over(&self) -> Option<&f64> {
    self.intrest_amount_carried_over.as_ref()
  }

  pub fn reset_intrest_amount_carried_over(&mut self) {
    self.intrest_amount_carried_over = None;
  }

  pub fn set_penalty_amount_carried_over(&mut self, penalty_amount_carried_over: f64) {
    self.penalty_amount_carried_over = Some(penalty_amount_carried_over);
  }

  pub fn with_penalty_amount_carried_over(mut self, penalty_amount_carried_over: f64) -> ScheduledPayment {
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

  pub fn with_total_amount(mut self, total_amount: f64) -> ScheduledPayment {
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



