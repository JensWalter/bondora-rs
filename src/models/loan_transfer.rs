#[derive(Debug, Serialize, Deserialize)]
pub struct LoanTransfer {
    /// Payment date
    #[serde(rename = "Date")]
    pub date: Option<String>,
    /// Principal amount
    #[serde(rename = "PrincipalAmount")]
    pub principal_amount: Option<f64>,
    /// Interest amount
    #[serde(rename = "InterestAmount")]
    pub interest_amount: Option<f64>,
    /// Interest carried over amount
    #[serde(rename = "InterestAmountCarriedOver")]
    pub interest_amount_carried_over: Option<f64>,
    /// Penalty amount
    #[serde(rename = "PenaltyAmountCarriedOver")]
    pub penalty_amount_carried_over: Option<f64>,
    /// Total amount
    #[serde(rename = "TotalAmount")]
    pub total_amount: Option<f64>,
}
