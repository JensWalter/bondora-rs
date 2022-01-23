/// MyAccountBalance : My account balance information
#[derive(Debug, Serialize, Deserialize)]
pub struct MyAccountBalance {
  /// Account balance
  #[serde(rename = "Balance")]
  pub balance: Option<f64>,
  /// Account reserved amount
  #[serde(rename = "Reserved")]
  pub reserved: Option<f64>,
  /// Api pending auction bid request amount
  #[serde(rename = "BidRequestAmount")]
  pub bid_request_amount: Option<i32>,
  /// Available balance
  #[serde(rename = "TotalAvailable")]
  pub total_available: Option<f64>,
  /// List of Go and Grow accounts
  #[serde(rename = "GoGrowAccounts")]
  pub go_grow_accounts: Option<Vec<crate::models::GoGrowAccount>>
}
