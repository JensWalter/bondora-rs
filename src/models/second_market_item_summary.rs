#[derive(Debug, Serialize, Deserialize)]
pub struct SecondMarketItemSummary {
    /// Item unique identifier
    #[serde(rename = "Id")]
    pub id: Option<String>,
    /// Number
    #[serde(rename = "Number")]
    pub number: Option<i32>,
    /// Item start date
    #[serde(rename = "StartDate")]
    pub start_date: Option<String>,
    /// Planned close date
    #[serde(rename = "PlannedCloseDate")]
    pub planned_close_date: Option<String>,
    /// Actual close date
    #[serde(rename = "ActualCloseDate")]
    pub actual_close_date: Option<String>,
    /// User cancelled on
    #[serde(rename = "UserCancelledOn")]
    pub user_cancelled_on: Option<String>,
    /// LoanPart being sold
    #[serde(rename = "LoanPart_id")]
    pub loan_part_id: Option<String>,
    /// Discount rate percent
    #[serde(rename = "DesiredDiscountRate")]
    pub desired_discount_rate: Option<f64>,
    /// Discount rate as fraction (0.0 - 1.0)
    #[serde(rename = "DesiredDiscountRateDecimalFraction")]
    pub desired_discount_rate_decimal_fraction: Option<f64>,
    /// Current status code  <para>0 Created</para><para>1 Open in marketplace</para><para>2 Successfully sold</para><para>3 Failed</para>
    #[serde(rename = "StatusCode")]
    pub status_code: Option<i32>,
}
