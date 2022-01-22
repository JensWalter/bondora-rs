
/// Debt : Borrower's debt

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct Debt {
  /// Start
  #[serde(rename = "StartDate")]
  start_date: Option<String>,
  /// End
  #[serde(rename = "EndDate")]
  end_date: Option<String>,
  /// Amount
  #[serde(rename = "Amount")]
  amount: Option<String>,
  /// Max amount
  #[serde(rename = "MaxAmount")]
  max_amount: Option<String>,
  /// Industry
  #[serde(rename = "Industry")]
  industry: Option<String>,
  /// Reporter
  #[serde(rename = "Reporter")]
  reporter: Option<String>
}

impl Debt {
  /// Borrower's debt
  pub fn new() -> Debt {
    Debt {
      start_date: None,
      end_date: None,
      amount: None,
      max_amount: None,
      industry: None,
      reporter: None
    }
  }

  pub fn set_start_date(&mut self, start_date: String) {
    self.start_date = Some(start_date);
  }

  pub fn with_start_date(mut self, start_date: String) -> Debt {
    self.start_date = Some(start_date);
    self
  }

  pub fn start_date(&self) -> Option<&String> {
    self.start_date.as_ref()
  }

  pub fn reset_start_date(&mut self) {
    self.start_date = None;
  }

  pub fn set_end_date(&mut self, end_date: String) {
    self.end_date = Some(end_date);
  }

  pub fn with_end_date(mut self, end_date: String) -> Debt {
    self.end_date = Some(end_date);
    self
  }

  pub fn end_date(&self) -> Option<&String> {
    self.end_date.as_ref()
  }

  pub fn reset_end_date(&mut self) {
    self.end_date = None;
  }

  pub fn set_amount(&mut self, amount: String) {
    self.amount = Some(amount);
  }

  pub fn with_amount(mut self, amount: String) -> Debt {
    self.amount = Some(amount);
    self
  }

  pub fn amount(&self) -> Option<&String> {
    self.amount.as_ref()
  }

  pub fn reset_amount(&mut self) {
    self.amount = None;
  }

  pub fn set_max_amount(&mut self, max_amount: String) {
    self.max_amount = Some(max_amount);
  }

  pub fn with_max_amount(mut self, max_amount: String) -> Debt {
    self.max_amount = Some(max_amount);
    self
  }

  pub fn max_amount(&self) -> Option<&String> {
    self.max_amount.as_ref()
  }

  pub fn reset_max_amount(&mut self) {
    self.max_amount = None;
  }

  pub fn set_industry(&mut self, industry: String) {
    self.industry = Some(industry);
  }

  pub fn with_industry(mut self, industry: String) -> Debt {
    self.industry = Some(industry);
    self
  }

  pub fn industry(&self) -> Option<&String> {
    self.industry.as_ref()
  }

  pub fn reset_industry(&mut self) {
    self.industry = None;
  }

  pub fn set_reporter(&mut self, reporter: String) {
    self.reporter = Some(reporter);
  }

  pub fn with_reporter(mut self, reporter: String) -> Debt {
    self.reporter = Some(reporter);
    self
  }

  pub fn reporter(&self) -> Option<&String> {
    self.reporter.as_ref()
  }

  pub fn reset_reporter(&mut self) {
    self.reporter = None;
  }

}



