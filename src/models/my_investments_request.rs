
/// MyInvestmentsRequest : Request object for filtering my investments

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct MyInvestmentsRequest {
  /// Loan issued start date from
  #[serde(rename = "LoanIssuedDateFrom")]
  loan_issued_date_from: Option<String>,
  /// Loan issued start date to
  #[serde(rename = "LoanIssuedDateTo")]
  loan_issued_date_to: Option<String>,
  /// Remaining principal amount min
  #[serde(rename = "PrincipalMin")]
  principal_min: Option<f64>,
  /// Remaining principal amount max
  #[serde(rename = "PrincipalMax")]
  principal_max: Option<f64>,
  /// Interest rate min
  #[serde(rename = "InterestMin")]
  interest_min: Option<f64>,
  /// Interest rate max
  #[serde(rename = "InterestMax")]
  interest_max: Option<f64>,
  /// Loan lenght min
  #[serde(rename = "LengthMax")]
  length_max: Option<i32>,
  /// Loan lenght max
  #[serde(rename = "LengthMin")]
  length_min: Option<i32>,
  /// Principal debt amount min
  #[serde(rename = "LatePrincipalAmountMin")]
  late_principal_amount_min: Option<f64>,
  /// Principal debt amount max
  #[serde(rename = "LatePrincipalAmountMax")]
  late_principal_amount_max: Option<f64>,
  /// Principal debt started date from
  #[serde(rename = "DebtOccuredOnFrom")]
  debt_occured_on_from: Option<String>,
  /// Principal debt started date to
  #[serde(rename = "DebtOccuredOnTo")]
  debt_occured_on_to: Option<String>,
  /// Interest debt started date from
  #[serde(rename = "DebtOccuredOnForSecondaryFrom")]
  debt_occured_on_for_secondary_from: Option<String>,
  /// Interest debt started date to
  #[serde(rename = "DebtOccuredOnForSecondaryTo")]
  debt_occured_on_for_secondary_to: Option<String>,
  /// Defaulted date from
  #[serde(rename = "DefaultedDateFrom")]
  defaulted_date_from: Option<String>,
  /// Defaulted date to
  #[serde(rename = "DefaultedDateTo")]
  defaulted_date_to: Option<String>,
  /// Defaulted date from
  #[serde(rename = "RescheduledFrom")]
  rescheduled_from: Option<String>,
  /// Defaulted date to
  #[serde(rename = "RescheduledTo")]
  rescheduled_to: Option<String>,
  /// When it was sold on Secondary market from
  #[serde(rename = "SoldDateFrom")]
  sold_date_from: Option<String>,
  /// When it was sold on Secondary market to
  #[serde(rename = "SoldDateTo")]
  sold_date_to: Option<String>,
  /// When you received the investment Auctions/Secondary market from
  #[serde(rename = "PurchaseDateFrom")]
  purchase_date_from: Option<String>,
  /// When you received the investment Auctions/Secondary market to
  #[serde(rename = "PurchaseDateTo")]
  purchase_date_to: Option<String>,
  /// Next payment date to
  #[serde(rename = "NextPaymentDateTo")]
  next_payment_date_to: Option<String>,
  /// Next payment date from
  #[serde(rename = "NextPaymentDateFrom")]
  next_payment_date_from: Option<String>,
  /// Last payment date from
  #[serde(rename = "LastPaymentDateFrom")]
  last_payment_date_from: Option<String>,
  /// Last payment date to
  #[serde(rename = "LastPaymentDateTo")]
  last_payment_date_to: Option<String>,
  /// Two letter iso code for country of origin: EE, ES, FI
  #[serde(rename = "Countries")]
  countries: Option<Vec<String>>,
  /// Bondora's rating: AA, A, B, C, D, E, F, HR
  #[serde(rename = "Ratings")]
  ratings: Option<Vec<String>>,
  /// Minimum credit score
  #[serde(rename = "CreditScoreMin")]
  credit_score_min: Option<i32>,
  /// Maximum credit score
  #[serde(rename = "CreditScoreMax")]
  credit_score_max: Option<i32>,
  /// Borrower's username
  #[serde(rename = "UserName")]
  user_name: Option<String>,
  /// Loan status code  <para>0 Reserved</para><para>2 Current</para><para>3 Cancelled</para><para>100 Overdue</para><para>5 60+ days overdue</para><para>4 Repaid</para><para>8 Released</para>
  #[serde(rename = "LoanStatusCode")]
  loan_status_code: Option<Vec<i32>>,
  /// Income verification type
  #[serde(rename = "IncomeVerificationStatus")]
  income_verification_status: Option<i32>,
  /// Latest debt management stage
  #[serde(rename = "LoanDebtManagementStage")]
  loan_debt_management_stage: Option<i32>,
  /// Latest debt management stage type
  #[serde(rename = "LoanDebtManagementStageType")]
  loan_debt_management_stage_type: Option<i32>,
  /// Latest debt management date active from
  #[serde(rename = "LoanDebtManagementDateActiveFrom")]
  loan_debt_management_date_active_from: Option<String>,
  /// Latest debt management date active to
  #[serde(rename = "LoanDebtManagementDateActiveTo")]
  loan_debt_management_date_active_to: Option<String>,
  /// Auction bid type
  #[serde(rename = "AuctionBidType")]
  auction_bid_type: Option<i32>,
  /// Second market sale status  <para>NULL All active</para><para>0 Bought investments</para><para>1 Sold investments</para><para>2 Investment is on sale</para><para>3 Investment is not on sale</para>
  #[serde(rename = "SalesStatus")]
  sales_status: Option<i32>,
  /// Search only active in repayment loans, StatusCodes (2, 5, 100)
  #[serde(rename = "IsInRepayment")]
  is_in_repayment: Option<bool>,
  /// Max items in result, up to 50000
  #[serde(rename = "PageSize")]
  page_size: Option<i32>,
  /// Result page nr
  #[serde(rename = "PageNr")]
  page_nr: Option<i32>
}

impl MyInvestmentsRequest {
  /// Request object for filtering my investments
  pub fn new() -> MyInvestmentsRequest {
    MyInvestmentsRequest {
      loan_issued_date_from: None,
      loan_issued_date_to: None,
      principal_min: None,
      principal_max: None,
      interest_min: None,
      interest_max: None,
      length_max: None,
      length_min: None,
      late_principal_amount_min: None,
      late_principal_amount_max: None,
      debt_occured_on_from: None,
      debt_occured_on_to: None,
      debt_occured_on_for_secondary_from: None,
      debt_occured_on_for_secondary_to: None,
      defaulted_date_from: None,
      defaulted_date_to: None,
      rescheduled_from: None,
      rescheduled_to: None,
      sold_date_from: None,
      sold_date_to: None,
      purchase_date_from: None,
      purchase_date_to: None,
      next_payment_date_to: None,
      next_payment_date_from: None,
      last_payment_date_from: None,
      last_payment_date_to: None,
      countries: None,
      ratings: None,
      credit_score_min: None,
      credit_score_max: None,
      user_name: None,
      loan_status_code: None,
      income_verification_status: None,
      loan_debt_management_stage: None,
      loan_debt_management_stage_type: None,
      loan_debt_management_date_active_from: None,
      loan_debt_management_date_active_to: None,
      auction_bid_type: None,
      sales_status: None,
      is_in_repayment: None,
      page_size: None,
      page_nr: None
    }
  }

  pub fn set_loan_issued_date_from(&mut self, loan_issued_date_from: String) {
    self.loan_issued_date_from = Some(loan_issued_date_from);
  }

  pub fn with_loan_issued_date_from(mut self, loan_issued_date_from: String) -> MyInvestmentsRequest {
    self.loan_issued_date_from = Some(loan_issued_date_from);
    self
  }

  pub fn loan_issued_date_from(&self) -> Option<&String> {
    self.loan_issued_date_from.as_ref()
  }

  pub fn reset_loan_issued_date_from(&mut self) {
    self.loan_issued_date_from = None;
  }

  pub fn set_loan_issued_date_to(&mut self, loan_issued_date_to: String) {
    self.loan_issued_date_to = Some(loan_issued_date_to);
  }

  pub fn with_loan_issued_date_to(mut self, loan_issued_date_to: String) -> MyInvestmentsRequest {
    self.loan_issued_date_to = Some(loan_issued_date_to);
    self
  }

  pub fn loan_issued_date_to(&self) -> Option<&String> {
    self.loan_issued_date_to.as_ref()
  }

  pub fn reset_loan_issued_date_to(&mut self) {
    self.loan_issued_date_to = None;
  }

  pub fn set_principal_min(&mut self, principal_min: f64) {
    self.principal_min = Some(principal_min);
  }

  pub fn with_principal_min(mut self, principal_min: f64) -> MyInvestmentsRequest {
    self.principal_min = Some(principal_min);
    self
  }

  pub fn principal_min(&self) -> Option<&f64> {
    self.principal_min.as_ref()
  }

  pub fn reset_principal_min(&mut self) {
    self.principal_min = None;
  }

  pub fn set_principal_max(&mut self, principal_max: f64) {
    self.principal_max = Some(principal_max);
  }

  pub fn with_principal_max(mut self, principal_max: f64) -> MyInvestmentsRequest {
    self.principal_max = Some(principal_max);
    self
  }

  pub fn principal_max(&self) -> Option<&f64> {
    self.principal_max.as_ref()
  }

  pub fn reset_principal_max(&mut self) {
    self.principal_max = None;
  }

  pub fn set_interest_min(&mut self, interest_min: f64) {
    self.interest_min = Some(interest_min);
  }

  pub fn with_interest_min(mut self, interest_min: f64) -> MyInvestmentsRequest {
    self.interest_min = Some(interest_min);
    self
  }

  pub fn interest_min(&self) -> Option<&f64> {
    self.interest_min.as_ref()
  }

  pub fn reset_interest_min(&mut self) {
    self.interest_min = None;
  }

  pub fn set_interest_max(&mut self, interest_max: f64) {
    self.interest_max = Some(interest_max);
  }

  pub fn with_interest_max(mut self, interest_max: f64) -> MyInvestmentsRequest {
    self.interest_max = Some(interest_max);
    self
  }

  pub fn interest_max(&self) -> Option<&f64> {
    self.interest_max.as_ref()
  }

  pub fn reset_interest_max(&mut self) {
    self.interest_max = None;
  }

  pub fn set_length_max(&mut self, length_max: i32) {
    self.length_max = Some(length_max);
  }

  pub fn with_length_max(mut self, length_max: i32) -> MyInvestmentsRequest {
    self.length_max = Some(length_max);
    self
  }

  pub fn length_max(&self) -> Option<&i32> {
    self.length_max.as_ref()
  }

  pub fn reset_length_max(&mut self) {
    self.length_max = None;
  }

  pub fn set_length_min(&mut self, length_min: i32) {
    self.length_min = Some(length_min);
  }

  pub fn with_length_min(mut self, length_min: i32) -> MyInvestmentsRequest {
    self.length_min = Some(length_min);
    self
  }

  pub fn length_min(&self) -> Option<&i32> {
    self.length_min.as_ref()
  }

  pub fn reset_length_min(&mut self) {
    self.length_min = None;
  }

  pub fn set_late_principal_amount_min(&mut self, late_principal_amount_min: f64) {
    self.late_principal_amount_min = Some(late_principal_amount_min);
  }

  pub fn with_late_principal_amount_min(mut self, late_principal_amount_min: f64) -> MyInvestmentsRequest {
    self.late_principal_amount_min = Some(late_principal_amount_min);
    self
  }

  pub fn late_principal_amount_min(&self) -> Option<&f64> {
    self.late_principal_amount_min.as_ref()
  }

  pub fn reset_late_principal_amount_min(&mut self) {
    self.late_principal_amount_min = None;
  }

  pub fn set_late_principal_amount_max(&mut self, late_principal_amount_max: f64) {
    self.late_principal_amount_max = Some(late_principal_amount_max);
  }

  pub fn with_late_principal_amount_max(mut self, late_principal_amount_max: f64) -> MyInvestmentsRequest {
    self.late_principal_amount_max = Some(late_principal_amount_max);
    self
  }

  pub fn late_principal_amount_max(&self) -> Option<&f64> {
    self.late_principal_amount_max.as_ref()
  }

  pub fn reset_late_principal_amount_max(&mut self) {
    self.late_principal_amount_max = None;
  }

  pub fn set_debt_occured_on_from(&mut self, debt_occured_on_from: String) {
    self.debt_occured_on_from = Some(debt_occured_on_from);
  }

  pub fn with_debt_occured_on_from(mut self, debt_occured_on_from: String) -> MyInvestmentsRequest {
    self.debt_occured_on_from = Some(debt_occured_on_from);
    self
  }

  pub fn debt_occured_on_from(&self) -> Option<&String> {
    self.debt_occured_on_from.as_ref()
  }

  pub fn reset_debt_occured_on_from(&mut self) {
    self.debt_occured_on_from = None;
  }

  pub fn set_debt_occured_on_to(&mut self, debt_occured_on_to: String) {
    self.debt_occured_on_to = Some(debt_occured_on_to);
  }

  pub fn with_debt_occured_on_to(mut self, debt_occured_on_to: String) -> MyInvestmentsRequest {
    self.debt_occured_on_to = Some(debt_occured_on_to);
    self
  }

  pub fn debt_occured_on_to(&self) -> Option<&String> {
    self.debt_occured_on_to.as_ref()
  }

  pub fn reset_debt_occured_on_to(&mut self) {
    self.debt_occured_on_to = None;
  }

  pub fn set_debt_occured_on_for_secondary_from(&mut self, debt_occured_on_for_secondary_from: String) {
    self.debt_occured_on_for_secondary_from = Some(debt_occured_on_for_secondary_from);
  }

  pub fn with_debt_occured_on_for_secondary_from(mut self, debt_occured_on_for_secondary_from: String) -> MyInvestmentsRequest {
    self.debt_occured_on_for_secondary_from = Some(debt_occured_on_for_secondary_from);
    self
  }

  pub fn debt_occured_on_for_secondary_from(&self) -> Option<&String> {
    self.debt_occured_on_for_secondary_from.as_ref()
  }

  pub fn reset_debt_occured_on_for_secondary_from(&mut self) {
    self.debt_occured_on_for_secondary_from = None;
  }

  pub fn set_debt_occured_on_for_secondary_to(&mut self, debt_occured_on_for_secondary_to: String) {
    self.debt_occured_on_for_secondary_to = Some(debt_occured_on_for_secondary_to);
  }

  pub fn with_debt_occured_on_for_secondary_to(mut self, debt_occured_on_for_secondary_to: String) -> MyInvestmentsRequest {
    self.debt_occured_on_for_secondary_to = Some(debt_occured_on_for_secondary_to);
    self
  }

  pub fn debt_occured_on_for_secondary_to(&self) -> Option<&String> {
    self.debt_occured_on_for_secondary_to.as_ref()
  }

  pub fn reset_debt_occured_on_for_secondary_to(&mut self) {
    self.debt_occured_on_for_secondary_to = None;
  }

  pub fn set_defaulted_date_from(&mut self, defaulted_date_from: String) {
    self.defaulted_date_from = Some(defaulted_date_from);
  }

  pub fn with_defaulted_date_from(mut self, defaulted_date_from: String) -> MyInvestmentsRequest {
    self.defaulted_date_from = Some(defaulted_date_from);
    self
  }

  pub fn defaulted_date_from(&self) -> Option<&String> {
    self.defaulted_date_from.as_ref()
  }

  pub fn reset_defaulted_date_from(&mut self) {
    self.defaulted_date_from = None;
  }

  pub fn set_defaulted_date_to(&mut self, defaulted_date_to: String) {
    self.defaulted_date_to = Some(defaulted_date_to);
  }

  pub fn with_defaulted_date_to(mut self, defaulted_date_to: String) -> MyInvestmentsRequest {
    self.defaulted_date_to = Some(defaulted_date_to);
    self
  }

  pub fn defaulted_date_to(&self) -> Option<&String> {
    self.defaulted_date_to.as_ref()
  }

  pub fn reset_defaulted_date_to(&mut self) {
    self.defaulted_date_to = None;
  }

  pub fn set_rescheduled_from(&mut self, rescheduled_from: String) {
    self.rescheduled_from = Some(rescheduled_from);
  }

  pub fn with_rescheduled_from(mut self, rescheduled_from: String) -> MyInvestmentsRequest {
    self.rescheduled_from = Some(rescheduled_from);
    self
  }

  pub fn rescheduled_from(&self) -> Option<&String> {
    self.rescheduled_from.as_ref()
  }

  pub fn reset_rescheduled_from(&mut self) {
    self.rescheduled_from = None;
  }

  pub fn set_rescheduled_to(&mut self, rescheduled_to: String) {
    self.rescheduled_to = Some(rescheduled_to);
  }

  pub fn with_rescheduled_to(mut self, rescheduled_to: String) -> MyInvestmentsRequest {
    self.rescheduled_to = Some(rescheduled_to);
    self
  }

  pub fn rescheduled_to(&self) -> Option<&String> {
    self.rescheduled_to.as_ref()
  }

  pub fn reset_rescheduled_to(&mut self) {
    self.rescheduled_to = None;
  }

  pub fn set_sold_date_from(&mut self, sold_date_from: String) {
    self.sold_date_from = Some(sold_date_from);
  }

  pub fn with_sold_date_from(mut self, sold_date_from: String) -> MyInvestmentsRequest {
    self.sold_date_from = Some(sold_date_from);
    self
  }

  pub fn sold_date_from(&self) -> Option<&String> {
    self.sold_date_from.as_ref()
  }

  pub fn reset_sold_date_from(&mut self) {
    self.sold_date_from = None;
  }

  pub fn set_sold_date_to(&mut self, sold_date_to: String) {
    self.sold_date_to = Some(sold_date_to);
  }

  pub fn with_sold_date_to(mut self, sold_date_to: String) -> MyInvestmentsRequest {
    self.sold_date_to = Some(sold_date_to);
    self
  }

  pub fn sold_date_to(&self) -> Option<&String> {
    self.sold_date_to.as_ref()
  }

  pub fn reset_sold_date_to(&mut self) {
    self.sold_date_to = None;
  }

  pub fn set_purchase_date_from(&mut self, purchase_date_from: String) {
    self.purchase_date_from = Some(purchase_date_from);
  }

  pub fn with_purchase_date_from(mut self, purchase_date_from: String) -> MyInvestmentsRequest {
    self.purchase_date_from = Some(purchase_date_from);
    self
  }

  pub fn purchase_date_from(&self) -> Option<&String> {
    self.purchase_date_from.as_ref()
  }

  pub fn reset_purchase_date_from(&mut self) {
    self.purchase_date_from = None;
  }

  pub fn set_purchase_date_to(&mut self, purchase_date_to: String) {
    self.purchase_date_to = Some(purchase_date_to);
  }

  pub fn with_purchase_date_to(mut self, purchase_date_to: String) -> MyInvestmentsRequest {
    self.purchase_date_to = Some(purchase_date_to);
    self
  }

  pub fn purchase_date_to(&self) -> Option<&String> {
    self.purchase_date_to.as_ref()
  }

  pub fn reset_purchase_date_to(&mut self) {
    self.purchase_date_to = None;
  }

  pub fn set_next_payment_date_to(&mut self, next_payment_date_to: String) {
    self.next_payment_date_to = Some(next_payment_date_to);
  }

  pub fn with_next_payment_date_to(mut self, next_payment_date_to: String) -> MyInvestmentsRequest {
    self.next_payment_date_to = Some(next_payment_date_to);
    self
  }

  pub fn next_payment_date_to(&self) -> Option<&String> {
    self.next_payment_date_to.as_ref()
  }

  pub fn reset_next_payment_date_to(&mut self) {
    self.next_payment_date_to = None;
  }

  pub fn set_next_payment_date_from(&mut self, next_payment_date_from: String) {
    self.next_payment_date_from = Some(next_payment_date_from);
  }

  pub fn with_next_payment_date_from(mut self, next_payment_date_from: String) -> MyInvestmentsRequest {
    self.next_payment_date_from = Some(next_payment_date_from);
    self
  }

  pub fn next_payment_date_from(&self) -> Option<&String> {
    self.next_payment_date_from.as_ref()
  }

  pub fn reset_next_payment_date_from(&mut self) {
    self.next_payment_date_from = None;
  }

  pub fn set_last_payment_date_from(&mut self, last_payment_date_from: String) {
    self.last_payment_date_from = Some(last_payment_date_from);
  }

  pub fn with_last_payment_date_from(mut self, last_payment_date_from: String) -> MyInvestmentsRequest {
    self.last_payment_date_from = Some(last_payment_date_from);
    self
  }

  pub fn last_payment_date_from(&self) -> Option<&String> {
    self.last_payment_date_from.as_ref()
  }

  pub fn reset_last_payment_date_from(&mut self) {
    self.last_payment_date_from = None;
  }

  pub fn set_last_payment_date_to(&mut self, last_payment_date_to: String) {
    self.last_payment_date_to = Some(last_payment_date_to);
  }

  pub fn with_last_payment_date_to(mut self, last_payment_date_to: String) -> MyInvestmentsRequest {
    self.last_payment_date_to = Some(last_payment_date_to);
    self
  }

  pub fn last_payment_date_to(&self) -> Option<&String> {
    self.last_payment_date_to.as_ref()
  }

  pub fn reset_last_payment_date_to(&mut self) {
    self.last_payment_date_to = None;
  }

  pub fn set_countries(&mut self, countries: Vec<String>) {
    self.countries = Some(countries);
  }

  pub fn with_countries(mut self, countries: Vec<String>) -> MyInvestmentsRequest {
    self.countries = Some(countries);
    self
  }

  pub fn countries(&self) -> Option<&Vec<String>> {
    self.countries.as_ref()
  }

  pub fn reset_countries(&mut self) {
    self.countries = None;
  }

  pub fn set_ratings(&mut self, ratings: Vec<String>) {
    self.ratings = Some(ratings);
  }

  pub fn with_ratings(mut self, ratings: Vec<String>) -> MyInvestmentsRequest {
    self.ratings = Some(ratings);
    self
  }

  pub fn ratings(&self) -> Option<&Vec<String>> {
    self.ratings.as_ref()
  }

  pub fn reset_ratings(&mut self) {
    self.ratings = None;
  }

  pub fn set_credit_score_min(&mut self, credit_score_min: i32) {
    self.credit_score_min = Some(credit_score_min);
  }

  pub fn with_credit_score_min(mut self, credit_score_min: i32) -> MyInvestmentsRequest {
    self.credit_score_min = Some(credit_score_min);
    self
  }

  pub fn credit_score_min(&self) -> Option<&i32> {
    self.credit_score_min.as_ref()
  }

  pub fn reset_credit_score_min(&mut self) {
    self.credit_score_min = None;
  }

  pub fn set_credit_score_max(&mut self, credit_score_max: i32) {
    self.credit_score_max = Some(credit_score_max);
  }

  pub fn with_credit_score_max(mut self, credit_score_max: i32) -> MyInvestmentsRequest {
    self.credit_score_max = Some(credit_score_max);
    self
  }

  pub fn credit_score_max(&self) -> Option<&i32> {
    self.credit_score_max.as_ref()
  }

  pub fn reset_credit_score_max(&mut self) {
    self.credit_score_max = None;
  }

  pub fn set_user_name(&mut self, user_name: String) {
    self.user_name = Some(user_name);
  }

  pub fn with_user_name(mut self, user_name: String) -> MyInvestmentsRequest {
    self.user_name = Some(user_name);
    self
  }

  pub fn user_name(&self) -> Option<&String> {
    self.user_name.as_ref()
  }

  pub fn reset_user_name(&mut self) {
    self.user_name = None;
  }

  pub fn set_loan_status_code(&mut self, loan_status_code: Vec<i32>) {
    self.loan_status_code = Some(loan_status_code);
  }

  pub fn with_loan_status_code(mut self, loan_status_code: Vec<i32>) -> MyInvestmentsRequest {
    self.loan_status_code = Some(loan_status_code);
    self
  }

  pub fn loan_status_code(&self) -> Option<&Vec<i32>> {
    self.loan_status_code.as_ref()
  }

  pub fn reset_loan_status_code(&mut self) {
    self.loan_status_code = None;
  }

  pub fn set_income_verification_status(&mut self, income_verification_status: i32) {
    self.income_verification_status = Some(income_verification_status);
  }

  pub fn with_income_verification_status(mut self, income_verification_status: i32) -> MyInvestmentsRequest {
    self.income_verification_status = Some(income_verification_status);
    self
  }

  pub fn income_verification_status(&self) -> Option<&i32> {
    self.income_verification_status.as_ref()
  }

  pub fn reset_income_verification_status(&mut self) {
    self.income_verification_status = None;
  }

  pub fn set_loan_debt_management_stage(&mut self, loan_debt_management_stage: i32) {
    self.loan_debt_management_stage = Some(loan_debt_management_stage);
  }

  pub fn with_loan_debt_management_stage(mut self, loan_debt_management_stage: i32) -> MyInvestmentsRequest {
    self.loan_debt_management_stage = Some(loan_debt_management_stage);
    self
  }

  pub fn loan_debt_management_stage(&self) -> Option<&i32> {
    self.loan_debt_management_stage.as_ref()
  }

  pub fn reset_loan_debt_management_stage(&mut self) {
    self.loan_debt_management_stage = None;
  }

  pub fn set_loan_debt_management_stage_type(&mut self, loan_debt_management_stage_type: i32) {
    self.loan_debt_management_stage_type = Some(loan_debt_management_stage_type);
  }

  pub fn with_loan_debt_management_stage_type(mut self, loan_debt_management_stage_type: i32) -> MyInvestmentsRequest {
    self.loan_debt_management_stage_type = Some(loan_debt_management_stage_type);
    self
  }

  pub fn loan_debt_management_stage_type(&self) -> Option<&i32> {
    self.loan_debt_management_stage_type.as_ref()
  }

  pub fn reset_loan_debt_management_stage_type(&mut self) {
    self.loan_debt_management_stage_type = None;
  }

  pub fn set_loan_debt_management_date_active_from(&mut self, loan_debt_management_date_active_from: String) {
    self.loan_debt_management_date_active_from = Some(loan_debt_management_date_active_from);
  }

  pub fn with_loan_debt_management_date_active_from(mut self, loan_debt_management_date_active_from: String) -> MyInvestmentsRequest {
    self.loan_debt_management_date_active_from = Some(loan_debt_management_date_active_from);
    self
  }

  pub fn loan_debt_management_date_active_from(&self) -> Option<&String> {
    self.loan_debt_management_date_active_from.as_ref()
  }

  pub fn reset_loan_debt_management_date_active_from(&mut self) {
    self.loan_debt_management_date_active_from = None;
  }

  pub fn set_loan_debt_management_date_active_to(&mut self, loan_debt_management_date_active_to: String) {
    self.loan_debt_management_date_active_to = Some(loan_debt_management_date_active_to);
  }

  pub fn with_loan_debt_management_date_active_to(mut self, loan_debt_management_date_active_to: String) -> MyInvestmentsRequest {
    self.loan_debt_management_date_active_to = Some(loan_debt_management_date_active_to);
    self
  }

  pub fn loan_debt_management_date_active_to(&self) -> Option<&String> {
    self.loan_debt_management_date_active_to.as_ref()
  }

  pub fn reset_loan_debt_management_date_active_to(&mut self) {
    self.loan_debt_management_date_active_to = None;
  }

  pub fn set_auction_bid_type(&mut self, auction_bid_type: i32) {
    self.auction_bid_type = Some(auction_bid_type);
  }

  pub fn with_auction_bid_type(mut self, auction_bid_type: i32) -> MyInvestmentsRequest {
    self.auction_bid_type = Some(auction_bid_type);
    self
  }

  pub fn auction_bid_type(&self) -> Option<&i32> {
    self.auction_bid_type.as_ref()
  }

  pub fn reset_auction_bid_type(&mut self) {
    self.auction_bid_type = None;
  }

  pub fn set_sales_status(&mut self, sales_status: i32) {
    self.sales_status = Some(sales_status);
  }

  pub fn with_sales_status(mut self, sales_status: i32) -> MyInvestmentsRequest {
    self.sales_status = Some(sales_status);
    self
  }

  pub fn sales_status(&self) -> Option<&i32> {
    self.sales_status.as_ref()
  }

  pub fn reset_sales_status(&mut self) {
    self.sales_status = None;
  }

  pub fn set_is_in_repayment(&mut self, is_in_repayment: bool) {
    self.is_in_repayment = Some(is_in_repayment);
  }

  pub fn with_is_in_repayment(mut self, is_in_repayment: bool) -> MyInvestmentsRequest {
    self.is_in_repayment = Some(is_in_repayment);
    self
  }

  pub fn is_in_repayment(&self) -> Option<&bool> {
    self.is_in_repayment.as_ref()
  }

  pub fn reset_is_in_repayment(&mut self) {
    self.is_in_repayment = None;
  }

  pub fn set_page_size(&mut self, page_size: i32) {
    self.page_size = Some(page_size);
  }

  pub fn with_page_size(mut self, page_size: i32) -> MyInvestmentsRequest {
    self.page_size = Some(page_size);
    self
  }

  pub fn page_size(&self) -> Option<&i32> {
    self.page_size.as_ref()
  }

  pub fn reset_page_size(&mut self) {
    self.page_size = None;
  }

  pub fn set_page_nr(&mut self, page_nr: i32) {
    self.page_nr = Some(page_nr);
  }

  pub fn with_page_nr(mut self, page_nr: i32) -> MyInvestmentsRequest {
    self.page_nr = Some(page_nr);
    self
  }

  pub fn page_nr(&self) -> Option<&i32> {
    self.page_nr.as_ref()
  }

  pub fn reset_page_nr(&mut self) {
    self.page_nr = None;
  }

}



