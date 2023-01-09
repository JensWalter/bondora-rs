/// BorrowerHistory : Borrower's history
#[derive(Debug, Serialize, Deserialize)]
pub struct BorrowerHistory {
    /// Borrower's current overdue amount
    #[serde(rename = "Overdue")]
    pub overdue: Option<f64>,
    /// Borrower's total principal repaid
    #[serde(rename = "PrincipalRepaid")]
    pub principal_repaid: Option<f64>,
    /// Borrower's total interest paid
    #[serde(rename = "InterestRepaid")]
    pub interest_repaid: Option<f64>,
    /// Borrower's total late charges paid
    #[serde(rename = "LateChargesRepaid")]
    pub late_charges_repaid: Option<f64>,
    /// Borrower's total repaiments
    #[serde(rename = "RepaimentsTotal")]
    pub repaiments_total: Option<f64>,
    /// Borrower's issued loans count
    #[serde(rename = "IssuedLoans")]
    pub issued_loans: Option<i32>,
    /// Borrower's issued loans amount
    #[serde(rename = "IssuedLoanAmount")]
    pub issued_loan_amount: Option<f64>,
}
