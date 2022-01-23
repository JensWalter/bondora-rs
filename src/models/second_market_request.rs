/// SecondMarketRequest : Request object for filtering secondary market
#[derive(Debug, Serialize, Deserialize)]
pub struct SecondMarketRequest {
  /// Loan issued start date from
  #[serde(rename = "LoanIssuedDateFrom")]
  pub loan_issued_date_from: Option<String>,
  /// Loan issued start date to
  #[serde(rename = "LoanIssuedDateTo")]
  pub loan_issued_date_to: Option<String>,
  /// Remaining principal amount min
  #[serde(rename = "PrincipalMin")]
  pub principal_min: Option<f64>,
  /// Remaining principal amount max
  #[serde(rename = "PrincipalMax")]
  pub principal_max: Option<f64>,
  /// Interest rate min
  #[serde(rename = "InterestMin")]
  pub interest_min: Option<f64>,
  /// Interest rate max
  #[serde(rename = "InterestMax")]
  pub interest_max: Option<f64>,
  /// Loan lenght min
  #[serde(rename = "LengthMax")]
  pub length_max: Option<i32>,
  /// Loan lenght max
  #[serde(rename = "LengthMin")]
  pub length_min: Option<i32>,
  /// Is overdue
  #[serde(rename = "HasDebt")]
  pub has_debt: Option<bool>,
  /// Loan status code  <para>2 Current</para><para>100 Overdue</para><para>5 60+ days overdue</para>
  #[serde(rename = "LoanStatusCode")]
  pub loan_status_code: Option<Vec<i32>>,
  /// Latest debt management stage type
  #[serde(rename = "LoanDebtManagementStageType")]
  pub loan_debt_management_stage_type: Option<i32>,
  /// Latest debt management date active from
  #[serde(rename = "LoanDebtManagementDateActiveFrom")]
  pub loan_debt_management_date_active_from: Option<String>,
  /// Latest debt management date active to
  #[serde(rename = "LoanDebtManagementDateActiveTo")]
  pub loan_debt_management_date_active_to: Option<String>,
  /// Principal debt amount min
  #[serde(rename = "LatePrincipalAmountMin")]
  pub late_principal_amount_min: Option<f64>,
  /// Principal debt amount max
  #[serde(rename = "LatePrincipalAmountMax")]
  pub late_principal_amount_max: Option<f64>,
  /// Price amount min
  #[serde(rename = "PriceMin")]
  pub price_min: Option<f64>,
  /// Price amount max
  #[serde(rename = "PriceMax")]
  pub price_max: Option<f64>,
  /// Use of loan
  #[serde(rename = "UseOfLoan")]
  pub use_of_loan: Option<i32>,
  /// Has been rescheduled
  #[serde(rename = "HasNewSchedule")]
  pub has_new_schedule: Option<bool>,
  /// Two letter iso code for country of origin: EE, ES, FI
  #[serde(rename = "Countries")]
  pub countries: Option<Vec<String>>,
  /// Bondora's rating: AA, A, B, C, D, E, F, HR
  #[serde(rename = "Ratings")]
  pub ratings: Option<Vec<String>>,
  /// Minimum credit score
  #[serde(rename = "CreditScoreMin")]
  pub credit_score_min: Option<i32>,
  /// Maximum credit score
  #[serde(rename = "CreditScoreMax")]
  pub credit_score_max: Option<i32>,
  /// Borrower's username
  #[serde(rename = "UserName")]
  pub user_name: Option<String>,
  /// Borrower's gender: Male 0, Female 1, Unknown 2
  #[serde(rename = "Gender")]
  pub gender: Option<i32>,
  /// Minimal age
  #[serde(rename = "AgeMin")]
  pub age_min: Option<i32>,
  /// Maximum age
  #[serde(rename = "AgeMax")]
  pub age_max: Option<i32>,
  /// Income verification type
  #[serde(rename = "IncomeVerificationStatus")]
  pub income_verification_status: Option<i32>,
  /// Can find your own items from market: Value Null = ALL, True = only your items, False = other user items
  #[serde(rename = "ShowMyItems")]
  pub show_my_items: Option<bool>,
  /// Can find specific auction from market
  #[serde(rename = "AuctionId")]
  pub auction_id: Option<String>,
  /// Date when item was published from
  #[serde(rename = "ListedOnDateFrom")]
  pub listed_on_date_from: Option<String>,
  /// Date when item was published to
  #[serde(rename = "ListedOnDateTo")]
  pub listed_on_date_to: Option<String>,
  /// Principal debt started date from
  #[serde(rename = "DebtOccuredOnFrom")]
  pub debt_occured_on_from: Option<String>,
  /// Principal debt started date to
  #[serde(rename = "DebtOccuredOnTo")]
  pub debt_occured_on_to: Option<String>,
  /// Interest debt started date from
  #[serde(rename = "DebtOccuredOnForSecondaryFrom")]
  pub debt_occured_on_for_secondary_from: Option<String>,
  /// Interest debt started date to
  #[serde(rename = "DebtOccuredOnForSecondaryTo")]
  pub debt_occured_on_for_secondary_to: Option<String>,
  /// Defaulted date from
  #[serde(rename = "DefaultedDateFrom")]
  pub defaulted_date_from: Option<String>,
  /// Defaulted date to
  #[serde(rename = "DefaultedDateTo")]
  pub defaulted_date_to: Option<String>,
  /// Rescheduled date from
  #[serde(rename = "RescheduledFrom")]
  pub rescheduled_from: Option<String>,
  /// Rescheduled date to
  #[serde(rename = "RescheduledTo")]
  pub rescheduled_to: Option<String>,
  /// Last payment date from
  #[serde(rename = "LastPaymentDateFrom")]
  pub last_payment_date_from: Option<String>,
  /// Last payment date to
  #[serde(rename = "LastPaymentDateTo")]
  pub last_payment_date_to: Option<String>,
  /// Next payment date from
  #[serde(rename = "NextPaymentDateFrom")]
  pub next_payment_date_from: Option<String>,
  /// Next payment date to
  #[serde(rename = "NextPaymentDateTo")]
  pub next_payment_date_to: Option<String>,
  /// Minimal DesiredDiscountRate
  #[serde(rename = "DesiredDiscountRateMin")]
  pub desired_discount_rate_min: Option<f64>,
  /// Maximal DesiredDiscountRate
  #[serde(rename = "DesiredDiscountRateMax")]
  pub desired_discount_rate_max: Option<f64>,
  /// Minimal Xirr
  #[serde(rename = "XirrMin")]
  pub xirr_min: Option<f64>,
  /// Maximal Xirr
  #[serde(rename = "XirrMax")]
  pub xirr_max: Option<f64>,
  /// Max items in result, up to 100000
  #[serde(rename = "PageSize")]
  pub page_size: Option<i32>,
  /// Result page nr
  #[serde(rename = "PageNr")]
  pub page_nr: Option<i32>
}
