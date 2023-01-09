/// PublicDatasetRequest : Request parameters for getting loan public data
#[derive(Debug, Serialize, Deserialize)]
pub struct PublicDatasetRequest {
    /// Specific loans to search
    #[serde(rename = "LoanIds")]
    pub loan_ids: Option<Vec<String>>,
    /// Two letter iso code for country of origin: EE, ES, FI
    #[serde(rename = "Countries")]
    pub countries: Option<Vec<String>>,
    /// Bondora's rating: AA, A, B, C, D, E, F, HR
    #[serde(rename = "Ratings")]
    pub ratings: Option<Vec<String>>,
    /// Loan start date from
    #[serde(rename = "LoanDateFrom")]
    pub loan_date_from: Option<String>,
    /// Loan start date to
    #[serde(rename = "LoanDateTo")]
    pub loan_date_to: Option<String>,
    /// Max items in result, up to 10000
    #[serde(rename = "PageSize")]
    pub page_size: Option<i32>,
    /// Result page nr
    #[serde(rename = "PageNr")]
    pub page_nr: Option<i32>,
}
