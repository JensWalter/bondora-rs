
/// GoGrowAccount : Go and Grow account

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct GoGrowAccount {
  /// Name of your Go and Grow
  #[serde(rename = "Name")]
  name: Option<String>,
  /// TotalDeposits - TotalWithdrawals
  #[serde(rename = "NetDeposits")]
  net_deposits: Option<f64>,
  /// Everything you have gained from Go and Grow
  #[serde(rename = "NetProfit")]
  net_profit: Option<f64>,
  /// Total Go and Grow value
  #[serde(rename = "TotalSaved")]
  total_saved: Option<f64>
}

impl GoGrowAccount {
  /// Go and Grow account
  pub fn new() -> GoGrowAccount {
    GoGrowAccount {
      name: None,
      net_deposits: None,
      net_profit: None,
      total_saved: None
    }
  }

  pub fn set_name(&mut self, name: String) {
    self.name = Some(name);
  }

  pub fn with_name(mut self, name: String) -> GoGrowAccount {
    self.name = Some(name);
    self
  }

  pub fn name(&self) -> Option<&String> {
    self.name.as_ref()
  }

  pub fn reset_name(&mut self) {
    self.name = None;
  }

  pub fn set_net_deposits(&mut self, net_deposits: f64) {
    self.net_deposits = Some(net_deposits);
  }

  pub fn with_net_deposits(mut self, net_deposits: f64) -> GoGrowAccount {
    self.net_deposits = Some(net_deposits);
    self
  }

  pub fn net_deposits(&self) -> Option<&f64> {
    self.net_deposits.as_ref()
  }

  pub fn reset_net_deposits(&mut self) {
    self.net_deposits = None;
  }

  pub fn set_net_profit(&mut self, net_profit: f64) {
    self.net_profit = Some(net_profit);
  }

  pub fn with_net_profit(mut self, net_profit: f64) -> GoGrowAccount {
    self.net_profit = Some(net_profit);
    self
  }

  pub fn net_profit(&self) -> Option<&f64> {
    self.net_profit.as_ref()
  }

  pub fn reset_net_profit(&mut self) {
    self.net_profit = None;
  }

  pub fn set_total_saved(&mut self, total_saved: f64) {
    self.total_saved = Some(total_saved);
  }

  pub fn with_total_saved(mut self, total_saved: f64) -> GoGrowAccount {
    self.total_saved = Some(total_saved);
    self
  }

  pub fn total_saved(&self) -> Option<&f64> {
    self.total_saved.as_ref()
  }

  pub fn reset_total_saved(&mut self) {
    self.total_saved = None;
  }

}



