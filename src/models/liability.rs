
/// Liability : Borrower's liability

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct Liability {
  /// Is refinanced  <para>Only Value for new Auctions since 1st of june 2017 is false</para>
  #[serde(rename = "IsRefinanced")]
  is_refinanced: Option<bool>,
  /// Type of liability  <para>Only Value for new Auctions since 1st of june 2017 is -1 (NotUsed)</para>
  #[serde(rename = "TypeOfLiability")]
  type_of_liability: Option<i32>,
  /// Creditor
  #[serde(rename = "Creditor")]
  creditor: Option<String>,
  /// Monthly payment
  #[serde(rename = "MonthlyPayment")]
  monthly_payment: Option<f64>,
  /// Outstanding  <para>Only Value for new Auctions since 1st of june 2017 is NULL</para>
  #[serde(rename = "Outstanding")]
  outstanding: Option<f64>,
  /// Type of collateral  <para>Only Value for new Auctions since 1st of june 2017 is -1 (NotUsed)</para>
  #[serde(rename = "CollateralType")]
  collateral_type: Option<i32>
}

impl Liability {
  /// Borrower's liability
  pub fn new() -> Liability {
    Liability {
      is_refinanced: None,
      type_of_liability: None,
      creditor: None,
      monthly_payment: None,
      outstanding: None,
      collateral_type: None
    }
  }

  pub fn set_is_refinanced(&mut self, is_refinanced: bool) {
    self.is_refinanced = Some(is_refinanced);
  }

  pub fn with_is_refinanced(mut self, is_refinanced: bool) -> Liability {
    self.is_refinanced = Some(is_refinanced);
    self
  }

  pub fn is_refinanced(&self) -> Option<&bool> {
    self.is_refinanced.as_ref()
  }

  pub fn reset_is_refinanced(&mut self) {
    self.is_refinanced = None;
  }

  pub fn set_type_of_liability(&mut self, type_of_liability: i32) {
    self.type_of_liability = Some(type_of_liability);
  }

  pub fn with_type_of_liability(mut self, type_of_liability: i32) -> Liability {
    self.type_of_liability = Some(type_of_liability);
    self
  }

  pub fn type_of_liability(&self) -> Option<&i32> {
    self.type_of_liability.as_ref()
  }

  pub fn reset_type_of_liability(&mut self) {
    self.type_of_liability = None;
  }

  pub fn set_creditor(&mut self, creditor: String) {
    self.creditor = Some(creditor);
  }

  pub fn with_creditor(mut self, creditor: String) -> Liability {
    self.creditor = Some(creditor);
    self
  }

  pub fn creditor(&self) -> Option<&String> {
    self.creditor.as_ref()
  }

  pub fn reset_creditor(&mut self) {
    self.creditor = None;
  }

  pub fn set_monthly_payment(&mut self, monthly_payment: f64) {
    self.monthly_payment = Some(monthly_payment);
  }

  pub fn with_monthly_payment(mut self, monthly_payment: f64) -> Liability {
    self.monthly_payment = Some(monthly_payment);
    self
  }

  pub fn monthly_payment(&self) -> Option<&f64> {
    self.monthly_payment.as_ref()
  }

  pub fn reset_monthly_payment(&mut self) {
    self.monthly_payment = None;
  }

  pub fn set_outstanding(&mut self, outstanding: f64) {
    self.outstanding = Some(outstanding);
  }

  pub fn with_outstanding(mut self, outstanding: f64) -> Liability {
    self.outstanding = Some(outstanding);
    self
  }

  pub fn outstanding(&self) -> Option<&f64> {
    self.outstanding.as_ref()
  }

  pub fn reset_outstanding(&mut self) {
    self.outstanding = None;
  }

  pub fn set_collateral_type(&mut self, collateral_type: i32) {
    self.collateral_type = Some(collateral_type);
  }

  pub fn with_collateral_type(mut self, collateral_type: i32) -> Liability {
    self.collateral_type = Some(collateral_type);
    self
  }

  pub fn collateral_type(&self) -> Option<&i32> {
    self.collateral_type.as_ref()
  }

  pub fn reset_collateral_type(&mut self) {
    self.collateral_type = None;
  }

}



