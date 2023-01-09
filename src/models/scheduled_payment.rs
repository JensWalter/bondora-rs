/// ScheduledPayment : Scheduled payment item information
#[derive(Debug, Serialize, Deserialize)]
pub struct ScheduledPayment {
    /// Scheduled payment date
    #[serde(rename = "ScheduledDate")]
    pub scheduled_date: Option<String>,
    /// Scheduled principal amount
    #[serde(rename = "PrincipalAmount")]
    pub principal_amount: Option<f64>,
    /// Scheduled principal amount remaining after payment
    #[serde(rename = "PrincipalAmountLeft")]
    pub principal_amount_left: Option<f64>,
    /// Scheduled interest amount
    #[serde(rename = "InterestAmount")]
    pub interest_amount: Option<f64>,
    /// Interest amount carried over from rescheduling
    #[serde(rename = "IntrestAmountCarriedOver")]
    pub intrest_amount_carried_over: Option<f64>,
    /// Penalty amount carried over from rescheduling
    #[serde(rename = "PenaltyAmountCarriedOver")]
    pub penalty_amount_carried_over: Option<f64>,
    /// Total payment amount
    #[serde(rename = "TotalAmount")]
    pub total_amount: Option<f64>,
}
