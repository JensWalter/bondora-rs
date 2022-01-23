/// Debt : Borrower's debt
#[derive(Debug, Serialize, Deserialize)]
pub struct Debt {
  /// Start
  #[serde(rename = "StartDate")]
  pub start_date: Option<String>,
  /// End
  #[serde(rename = "EndDate")]
  pub end_date: Option<String>,
  /// Amount
  #[serde(rename = "Amount")]
  pub amount: Option<String>,
  /// Max amount
  #[serde(rename = "MaxAmount")]
  pub max_amount: Option<String>,
  /// Industry
  #[serde(rename = "Industry")]
  pub industry: Option<String>,
  /// Reporter
  #[serde(rename = "Reporter")]
  pub reporter: Option<String>
}
