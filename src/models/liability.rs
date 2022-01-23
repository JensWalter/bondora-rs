/// Liability : Borrower's liability
#[derive(Debug, Serialize, Deserialize)]
pub struct Liability {
  /// Is refinanced  <para>Only Value for new Auctions since 1st of june 2017 is false</para>
  #[serde(rename = "IsRefinanced")]
  pub is_refinanced: Option<bool>,
  /// Type of liability  <para>Only Value for new Auctions since 1st of june 2017 is -1 (NotUsed)</para>
  #[serde(rename = "TypeOfLiability")]
  pub type_of_liability: Option<i32>,
  /// Creditor
  #[serde(rename = "Creditor")]
  pub creditor: Option<String>,
  /// Monthly payment
  #[serde(rename = "MonthlyPayment")]
  pub monthly_payment: Option<f64>,
  /// Outstanding  <para>Only Value for new Auctions since 1st of june 2017 is NULL</para>
  #[serde(rename = "Outstanding")]
  pub outstanding: Option<f64>,
  /// Type of collateral  <para>Only Value for new Auctions since 1st of june 2017 is -1 (NotUsed)</para>
  #[serde(rename = "CollateralType")]
  pub collateral_type: Option<i32>
}
