
/// MyAccountBalance : My account balance information

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct MyAccountBalance {
  /// Account balance
  #[serde(rename = "Balance")]
  balance: Option<f64>,
  /// Account reserved amount
  #[serde(rename = "Reserved")]
  reserved: Option<f64>,
  /// Api pending auction bid request amount
  #[serde(rename = "BidRequestAmount")]
  bid_request_amount: Option<i32>,
  /// Available balance
  #[serde(rename = "TotalAvailable")]
  total_available: Option<f64>,
  /// List of Go and Grow accounts
  #[serde(rename = "GoGrowAccounts")]
  go_grow_accounts: Option<Vec<crate::models::GoGrowAccount>>
}

impl MyAccountBalance {
  /// My account balance information
  pub fn new() -> MyAccountBalance {
    MyAccountBalance {
      balance: None,
      reserved: None,
      bid_request_amount: None,
      total_available: None,
      go_grow_accounts: None
    }
  }

  pub fn set_balance(&mut self, balance: f64) {
    self.balance = Some(balance);
  }

  pub fn with_balance(mut self, balance: f64) -> MyAccountBalance {
    self.balance = Some(balance);
    self
  }

  pub fn balance(&self) -> Option<&f64> {
    self.balance.as_ref()
  }

  pub fn reset_balance(&mut self) {
    self.balance = None;
  }

  pub fn set_reserved(&mut self, reserved: f64) {
    self.reserved = Some(reserved);
  }

  pub fn with_reserved(mut self, reserved: f64) -> MyAccountBalance {
    self.reserved = Some(reserved);
    self
  }

  pub fn reserved(&self) -> Option<&f64> {
    self.reserved.as_ref()
  }

  pub fn reset_reserved(&mut self) {
    self.reserved = None;
  }

  pub fn set_bid_request_amount(&mut self, bid_request_amount: i32) {
    self.bid_request_amount = Some(bid_request_amount);
  }

  pub fn with_bid_request_amount(mut self, bid_request_amount: i32) -> MyAccountBalance {
    self.bid_request_amount = Some(bid_request_amount);
    self
  }

  pub fn bid_request_amount(&self) -> Option<&i32> {
    self.bid_request_amount.as_ref()
  }

  pub fn reset_bid_request_amount(&mut self) {
    self.bid_request_amount = None;
  }

  pub fn set_total_available(&mut self, total_available: f64) {
    self.total_available = Some(total_available);
  }

  pub fn with_total_available(mut self, total_available: f64) -> MyAccountBalance {
    self.total_available = Some(total_available);
    self
  }

  pub fn total_available(&self) -> Option<&f64> {
    self.total_available.as_ref()
  }

  pub fn reset_total_available(&mut self) {
    self.total_available = None;
  }

  pub fn set_go_grow_accounts(&mut self, go_grow_accounts: Vec<crate::models::GoGrowAccount>) {
    self.go_grow_accounts = Some(go_grow_accounts);
  }

  pub fn with_go_grow_accounts(mut self, go_grow_accounts: Vec<crate::models::GoGrowAccount>) -> MyAccountBalance {
    self.go_grow_accounts = Some(go_grow_accounts);
    self
  }

  pub fn go_grow_accounts(&self) -> Option<&Vec<crate::models::GoGrowAccount>> {
    self.go_grow_accounts.as_ref()
  }

  pub fn reset_go_grow_accounts(&mut self) {
    self.go_grow_accounts = None;
  }

}



