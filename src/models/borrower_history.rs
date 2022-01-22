
/// BorrowerHistory : Borrower's history

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct BorrowerHistory {
  /// Borrower's current overdue amount
  #[serde(rename = "Overdue")]
  overdue: Option<f64>,
  /// Borrower's total principal repaid
  #[serde(rename = "PrincipalRepaid")]
  principal_repaid: Option<f64>,
  /// Borrower's total interest paid
  #[serde(rename = "InterestRepaid")]
  interest_repaid: Option<f64>,
  /// Borrower's total late charges paid
  #[serde(rename = "LateChargesRepaid")]
  late_charges_repaid: Option<f64>,
  /// Borrower's total repaiments
  #[serde(rename = "RepaimentsTotal")]
  repaiments_total: Option<f64>,
  /// Borrower's issued loans count
  #[serde(rename = "IssuedLoans")]
  issued_loans: Option<i32>,
  /// Borrower's issued loans amount
  #[serde(rename = "IssuedLoanAmount")]
  issued_loan_amount: Option<f64>
}

impl BorrowerHistory {
  /// Borrower's history
  pub fn new() -> BorrowerHistory {
    BorrowerHistory {
      overdue: None,
      principal_repaid: None,
      interest_repaid: None,
      late_charges_repaid: None,
      repaiments_total: None,
      issued_loans: None,
      issued_loan_amount: None
    }
  }

  pub fn set_overdue(&mut self, overdue: f64) {
    self.overdue = Some(overdue);
  }

  pub fn with_overdue(mut self, overdue: f64) -> BorrowerHistory {
    self.overdue = Some(overdue);
    self
  }

  pub fn overdue(&self) -> Option<&f64> {
    self.overdue.as_ref()
  }

  pub fn reset_overdue(&mut self) {
    self.overdue = None;
  }

  pub fn set_principal_repaid(&mut self, principal_repaid: f64) {
    self.principal_repaid = Some(principal_repaid);
  }

  pub fn with_principal_repaid(mut self, principal_repaid: f64) -> BorrowerHistory {
    self.principal_repaid = Some(principal_repaid);
    self
  }

  pub fn principal_repaid(&self) -> Option<&f64> {
    self.principal_repaid.as_ref()
  }

  pub fn reset_principal_repaid(&mut self) {
    self.principal_repaid = None;
  }

  pub fn set_interest_repaid(&mut self, interest_repaid: f64) {
    self.interest_repaid = Some(interest_repaid);
  }

  pub fn with_interest_repaid(mut self, interest_repaid: f64) -> BorrowerHistory {
    self.interest_repaid = Some(interest_repaid);
    self
  }

  pub fn interest_repaid(&self) -> Option<&f64> {
    self.interest_repaid.as_ref()
  }

  pub fn reset_interest_repaid(&mut self) {
    self.interest_repaid = None;
  }

  pub fn set_late_charges_repaid(&mut self, late_charges_repaid: f64) {
    self.late_charges_repaid = Some(late_charges_repaid);
  }

  pub fn with_late_charges_repaid(mut self, late_charges_repaid: f64) -> BorrowerHistory {
    self.late_charges_repaid = Some(late_charges_repaid);
    self
  }

  pub fn late_charges_repaid(&self) -> Option<&f64> {
    self.late_charges_repaid.as_ref()
  }

  pub fn reset_late_charges_repaid(&mut self) {
    self.late_charges_repaid = None;
  }

  pub fn set_repaiments_total(&mut self, repaiments_total: f64) {
    self.repaiments_total = Some(repaiments_total);
  }

  pub fn with_repaiments_total(mut self, repaiments_total: f64) -> BorrowerHistory {
    self.repaiments_total = Some(repaiments_total);
    self
  }

  pub fn repaiments_total(&self) -> Option<&f64> {
    self.repaiments_total.as_ref()
  }

  pub fn reset_repaiments_total(&mut self) {
    self.repaiments_total = None;
  }

  pub fn set_issued_loans(&mut self, issued_loans: i32) {
    self.issued_loans = Some(issued_loans);
  }

  pub fn with_issued_loans(mut self, issued_loans: i32) -> BorrowerHistory {
    self.issued_loans = Some(issued_loans);
    self
  }

  pub fn issued_loans(&self) -> Option<&i32> {
    self.issued_loans.as_ref()
  }

  pub fn reset_issued_loans(&mut self) {
    self.issued_loans = None;
  }

  pub fn set_issued_loan_amount(&mut self, issued_loan_amount: f64) {
    self.issued_loan_amount = Some(issued_loan_amount);
  }

  pub fn with_issued_loan_amount(mut self, issued_loan_amount: f64) -> BorrowerHistory {
    self.issued_loan_amount = Some(issued_loan_amount);
    self
  }

  pub fn issued_loan_amount(&self) -> Option<&f64> {
    self.issued_loan_amount.as_ref()
  }

  pub fn reset_issued_loan_amount(&mut self) {
    self.issued_loan_amount = None;
  }

}



