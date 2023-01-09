/// GoGrowAccount : Go and Grow account
#[derive(Debug, Serialize, Deserialize)]
pub struct GoGrowAccount {
    /// Name of your Go and Grow
    #[serde(rename = "Name")]
    pub name: Option<String>,
    /// TotalDeposits - TotalWithdrawals
    #[serde(rename = "NetDeposits")]
    pub net_deposits: Option<f64>,
    /// Everything you have gained from Go and Grow
    #[serde(rename = "NetProfit")]
    pub net_profit: Option<f64>,
    /// Total Go and Grow value
    #[serde(rename = "TotalSaved")]
    pub total_saved: Option<f64>,
}
