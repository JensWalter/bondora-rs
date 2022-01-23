/// MyInvestmentsRequest : Request object for filtering my investments
#[derive(Debug, Serialize, Deserialize)]
pub struct MyInvestmentsRequest {
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
  /// Principal debt amount min
  #[serde(rename = "LatePrincipalAmountMin")]
  pub late_principal_amount_min: Option<f64>,
  /// Principal debt amount max
  #[serde(rename = "LatePrincipalAmountMax")]
  pub late_principal_amount_max: Option<f64>,
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
  /// Defaulted date from
  #[serde(rename = "RescheduledFrom")]
  pub rescheduled_from: Option<String>,
  /// Defaulted date to
  #[serde(rename = "RescheduledTo")]
  pub rescheduled_to: Option<String>,
  /// When it was sold on Secondary market from
  #[serde(rename = "SoldDateFrom")]
  pub sold_date_from: Option<String>,
  /// When it was sold on Secondary market to
  #[serde(rename = "SoldDateTo")]
  pub sold_date_to: Option<String>,
  /// When you received the investment Auctions/Secondary market from
  #[serde(rename = "PurchaseDateFrom")]
  pub purchase_date_from: Option<String>,
  /// When you received the investment Auctions/Secondary market to
  #[serde(rename = "PurchaseDateTo")]
  pub purchase_date_to: Option<String>,
  /// Next payment date to
  #[serde(rename = "NextPaymentDateTo")]
  pub next_payment_date_to: Option<String>,
  /// Next payment date from
  #[serde(rename = "NextPaymentDateFrom")]
  pub next_payment_date_from: Option<String>,
  /// Last payment date from
  #[serde(rename = "LastPaymentDateFrom")]
  pub last_payment_date_from: Option<String>,
  /// Last payment date to
  #[serde(rename = "LastPaymentDateTo")]
  pub last_payment_date_to: Option<String>,
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
  /// Loan status code  <para>0 Reserved</para><para>2 Current</para><para>3 Cancelled</para><para>100 Overdue</para><para>5 60+ days overdue</para><para>4 Repaid</para><para>8 Released</para>
  #[serde(rename = "LoanStatusCode")]
  pub loan_status_code: Option<Vec<i32>>,
  /// Income verification type
  #[serde(rename = "IncomeVerificationStatus")]
  pub income_verification_status: Option<i32>,
  /// Latest debt management stage
  #[serde(rename = "LoanDebtManagementStage")]
  pub loan_debt_management_stage: Option<i32>,
  /// Latest debt management stage type
  #[serde(rename = "LoanDebtManagementStageType")]
  pub loan_debt_management_stage_type: Option<i32>,
  /// Latest debt management date active from
  #[serde(rename = "LoanDebtManagementDateActiveFrom")]
  pub loan_debt_management_date_active_from: Option<String>,
  /// Latest debt management date active to
  #[serde(rename = "LoanDebtManagementDateActiveTo")]
  pub loan_debt_management_date_active_to: Option<String>,
  /// Auction bid type
  #[serde(rename = "AuctionBidType")]
  pub auction_bid_type: Option<i32>,
  /// Second market sale status  <para>NULL All active</para><para>0 Bought investments</para><para>1 Sold investments</para><para>2 Investment is on sale</para><para>3 Investment is not on sale</para>
  #[serde(rename = "SalesStatus")]
  pub sales_status: Option<i32>,
  /// Search only active in repayment loans, StatusCodes (2, 5, 100)
  #[serde(rename = "IsInRepayment")]
  pub is_in_repayment: Option<bool>,
  /// Max items in result, up to 50000
  #[serde(rename = "PageSize")]
  pub page_size: Option<i32>,
  /// Result page nr
  #[serde(rename = "PageNr")]
  pub page_nr: Option<i32>
}
