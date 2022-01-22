
/// SecondMarketRequest : Request object for filtering secondary market

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct SecondMarketRequest {
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
  /// Is overdue
  #[serde(rename = "HasDebt")]
  has_debt: Option<bool>,
  /// Loan status code  <para>2 Current</para><para>100 Overdue</para><para>5 60+ days overdue</para>
  #[serde(rename = "LoanStatusCode")]
  loan_status_code: Option<Vec<i32>>,
  /// Latest debt management stage type
  #[serde(rename = "LoanDebtManagementStageType")]
  loan_debt_management_stage_type: Option<i32>,
  /// Latest debt management date active from
  #[serde(rename = "LoanDebtManagementDateActiveFrom")]
  loan_debt_management_date_active_from: Option<String>,
  /// Latest debt management date active to
  #[serde(rename = "LoanDebtManagementDateActiveTo")]
  loan_debt_management_date_active_to: Option<String>,
  /// Principal debt amount min
  #[serde(rename = "LatePrincipalAmountMin")]
  late_principal_amount_min: Option<f64>,
  /// Principal debt amount max
  #[serde(rename = "LatePrincipalAmountMax")]
  late_principal_amount_max: Option<f64>,
  /// Price amount min
  #[serde(rename = "PriceMin")]
  price_min: Option<f64>,
  /// Price amount max
  #[serde(rename = "PriceMax")]
  price_max: Option<f64>,
  /// Use of loan
  #[serde(rename = "UseOfLoan")]
  use_of_loan: Option<i32>,
  /// Has been rescheduled
  #[serde(rename = "HasNewSchedule")]
  has_new_schedule: Option<bool>,
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
  /// Borrower's gender: Male 0, Female 1, Unknown 2
  #[serde(rename = "Gender")]
  gender: Option<i32>,
  /// Minimal age
  #[serde(rename = "AgeMin")]
  age_min: Option<i32>,
  /// Maximum age
  #[serde(rename = "AgeMax")]
  age_max: Option<i32>,
  /// Income verification type
  #[serde(rename = "IncomeVerificationStatus")]
  income_verification_status: Option<i32>,
  /// Can find your own items from market: Value Null = ALL, True = only your items, False = other user items
  #[serde(rename = "ShowMyItems")]
  show_my_items: Option<bool>,
  /// Can find specific auction from market
  #[serde(rename = "AuctionId")]
  auction_id: Option<String>,
  /// Date when item was published from
  #[serde(rename = "ListedOnDateFrom")]
  listed_on_date_from: Option<String>,
  /// Date when item was published to
  #[serde(rename = "ListedOnDateTo")]
  listed_on_date_to: Option<String>,
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
  /// Rescheduled date from
  #[serde(rename = "RescheduledFrom")]
  rescheduled_from: Option<String>,
  /// Rescheduled date to
  #[serde(rename = "RescheduledTo")]
  rescheduled_to: Option<String>,
  /// Last payment date from
  #[serde(rename = "LastPaymentDateFrom")]
  last_payment_date_from: Option<String>,
  /// Last payment date to
  #[serde(rename = "LastPaymentDateTo")]
  last_payment_date_to: Option<String>,
  /// Next payment date from
  #[serde(rename = "NextPaymentDateFrom")]
  next_payment_date_from: Option<String>,
  /// Next payment date to
  #[serde(rename = "NextPaymentDateTo")]
  next_payment_date_to: Option<String>,
  /// Minimal DesiredDiscountRate
  #[serde(rename = "DesiredDiscountRateMin")]
  desired_discount_rate_min: Option<f64>,
  /// Maximal DesiredDiscountRate
  #[serde(rename = "DesiredDiscountRateMax")]
  desired_discount_rate_max: Option<f64>,
  /// Minimal Xirr
  #[serde(rename = "XirrMin")]
  xirr_min: Option<f64>,
  /// Maximal Xirr
  #[serde(rename = "XirrMax")]
  xirr_max: Option<f64>,
  /// Max items in result, up to 100000
  #[serde(rename = "PageSize")]
  page_size: Option<i32>,
  /// Result page nr
  #[serde(rename = "PageNr")]
  page_nr: Option<i32>
}

impl SecondMarketRequest {
  /// Request object for filtering secondary market
  pub fn new() -> SecondMarketRequest {
    SecondMarketRequest {
      loan_issued_date_from: None,
      loan_issued_date_to: None,
      principal_min: None,
      principal_max: None,
      interest_min: None,
      interest_max: None,
      length_max: None,
      length_min: None,
      has_debt: None,
      loan_status_code: None,
      loan_debt_management_stage_type: None,
      loan_debt_management_date_active_from: None,
      loan_debt_management_date_active_to: None,
      late_principal_amount_min: None,
      late_principal_amount_max: None,
      price_min: None,
      price_max: None,
      use_of_loan: None,
      has_new_schedule: None,
      countries: None,
      ratings: None,
      credit_score_min: None,
      credit_score_max: None,
      user_name: None,
      gender: None,
      age_min: None,
      age_max: None,
      income_verification_status: None,
      show_my_items: None,
      auction_id: None,
      listed_on_date_from: None,
      listed_on_date_to: None,
      debt_occured_on_from: None,
      debt_occured_on_to: None,
      debt_occured_on_for_secondary_from: None,
      debt_occured_on_for_secondary_to: None,
      defaulted_date_from: None,
      defaulted_date_to: None,
      rescheduled_from: None,
      rescheduled_to: None,
      last_payment_date_from: None,
      last_payment_date_to: None,
      next_payment_date_from: None,
      next_payment_date_to: None,
      desired_discount_rate_min: None,
      desired_discount_rate_max: None,
      xirr_min: None,
      xirr_max: None,
      page_size: None,
      page_nr: None
    }
  }

  pub fn set_loan_issued_date_from(&mut self, loan_issued_date_from: String) {
    self.loan_issued_date_from = Some(loan_issued_date_from);
  }

  pub fn with_loan_issued_date_from(mut self, loan_issued_date_from: String) -> SecondMarketRequest {
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

  pub fn with_loan_issued_date_to(mut self, loan_issued_date_to: String) -> SecondMarketRequest {
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

  pub fn with_principal_min(mut self, principal_min: f64) -> SecondMarketRequest {
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

  pub fn with_principal_max(mut self, principal_max: f64) -> SecondMarketRequest {
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

  pub fn with_interest_min(mut self, interest_min: f64) -> SecondMarketRequest {
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

  pub fn with_interest_max(mut self, interest_max: f64) -> SecondMarketRequest {
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

  pub fn with_length_max(mut self, length_max: i32) -> SecondMarketRequest {
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

  pub fn with_length_min(mut self, length_min: i32) -> SecondMarketRequest {
    self.length_min = Some(length_min);
    self
  }

  pub fn length_min(&self) -> Option<&i32> {
    self.length_min.as_ref()
  }

  pub fn reset_length_min(&mut self) {
    self.length_min = None;
  }

  pub fn set_has_debt(&mut self, has_debt: bool) {
    self.has_debt = Some(has_debt);
  }

  pub fn with_has_debt(mut self, has_debt: bool) -> SecondMarketRequest {
    self.has_debt = Some(has_debt);
    self
  }

  pub fn has_debt(&self) -> Option<&bool> {
    self.has_debt.as_ref()
  }

  pub fn reset_has_debt(&mut self) {
    self.has_debt = None;
  }

  pub fn set_loan_status_code(&mut self, loan_status_code: Vec<i32>) {
    self.loan_status_code = Some(loan_status_code);
  }

  pub fn with_loan_status_code(mut self, loan_status_code: Vec<i32>) -> SecondMarketRequest {
    self.loan_status_code = Some(loan_status_code);
    self
  }

  pub fn loan_status_code(&self) -> Option<&Vec<i32>> {
    self.loan_status_code.as_ref()
  }

  pub fn reset_loan_status_code(&mut self) {
    self.loan_status_code = None;
  }

  pub fn set_loan_debt_management_stage_type(&mut self, loan_debt_management_stage_type: i32) {
    self.loan_debt_management_stage_type = Some(loan_debt_management_stage_type);
  }

  pub fn with_loan_debt_management_stage_type(mut self, loan_debt_management_stage_type: i32) -> SecondMarketRequest {
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

  pub fn with_loan_debt_management_date_active_from(mut self, loan_debt_management_date_active_from: String) -> SecondMarketRequest {
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

  pub fn with_loan_debt_management_date_active_to(mut self, loan_debt_management_date_active_to: String) -> SecondMarketRequest {
    self.loan_debt_management_date_active_to = Some(loan_debt_management_date_active_to);
    self
  }

  pub fn loan_debt_management_date_active_to(&self) -> Option<&String> {
    self.loan_debt_management_date_active_to.as_ref()
  }

  pub fn reset_loan_debt_management_date_active_to(&mut self) {
    self.loan_debt_management_date_active_to = None;
  }

  pub fn set_late_principal_amount_min(&mut self, late_principal_amount_min: f64) {
    self.late_principal_amount_min = Some(late_principal_amount_min);
  }

  pub fn with_late_principal_amount_min(mut self, late_principal_amount_min: f64) -> SecondMarketRequest {
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

  pub fn with_late_principal_amount_max(mut self, late_principal_amount_max: f64) -> SecondMarketRequest {
    self.late_principal_amount_max = Some(late_principal_amount_max);
    self
  }

  pub fn late_principal_amount_max(&self) -> Option<&f64> {
    self.late_principal_amount_max.as_ref()
  }

  pub fn reset_late_principal_amount_max(&mut self) {
    self.late_principal_amount_max = None;
  }

  pub fn set_price_min(&mut self, price_min: f64) {
    self.price_min = Some(price_min);
  }

  pub fn with_price_min(mut self, price_min: f64) -> SecondMarketRequest {
    self.price_min = Some(price_min);
    self
  }

  pub fn price_min(&self) -> Option<&f64> {
    self.price_min.as_ref()
  }

  pub fn reset_price_min(&mut self) {
    self.price_min = None;
  }

  pub fn set_price_max(&mut self, price_max: f64) {
    self.price_max = Some(price_max);
  }

  pub fn with_price_max(mut self, price_max: f64) -> SecondMarketRequest {
    self.price_max = Some(price_max);
    self
  }

  pub fn price_max(&self) -> Option<&f64> {
    self.price_max.as_ref()
  }

  pub fn reset_price_max(&mut self) {
    self.price_max = None;
  }

  pub fn set_use_of_loan(&mut self, use_of_loan: i32) {
    self.use_of_loan = Some(use_of_loan);
  }

  pub fn with_use_of_loan(mut self, use_of_loan: i32) -> SecondMarketRequest {
    self.use_of_loan = Some(use_of_loan);
    self
  }

  pub fn use_of_loan(&self) -> Option<&i32> {
    self.use_of_loan.as_ref()
  }

  pub fn reset_use_of_loan(&mut self) {
    self.use_of_loan = None;
  }

  pub fn set_has_new_schedule(&mut self, has_new_schedule: bool) {
    self.has_new_schedule = Some(has_new_schedule);
  }

  pub fn with_has_new_schedule(mut self, has_new_schedule: bool) -> SecondMarketRequest {
    self.has_new_schedule = Some(has_new_schedule);
    self
  }

  pub fn has_new_schedule(&self) -> Option<&bool> {
    self.has_new_schedule.as_ref()
  }

  pub fn reset_has_new_schedule(&mut self) {
    self.has_new_schedule = None;
  }

  pub fn set_countries(&mut self, countries: Vec<String>) {
    self.countries = Some(countries);
  }

  pub fn with_countries(mut self, countries: Vec<String>) -> SecondMarketRequest {
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

  pub fn with_ratings(mut self, ratings: Vec<String>) -> SecondMarketRequest {
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

  pub fn with_credit_score_min(mut self, credit_score_min: i32) -> SecondMarketRequest {
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

  pub fn with_credit_score_max(mut self, credit_score_max: i32) -> SecondMarketRequest {
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

  pub fn with_user_name(mut self, user_name: String) -> SecondMarketRequest {
    self.user_name = Some(user_name);
    self
  }

  pub fn user_name(&self) -> Option<&String> {
    self.user_name.as_ref()
  }

  pub fn reset_user_name(&mut self) {
    self.user_name = None;
  }

  pub fn set_gender(&mut self, gender: i32) {
    self.gender = Some(gender);
  }

  pub fn with_gender(mut self, gender: i32) -> SecondMarketRequest {
    self.gender = Some(gender);
    self
  }

  pub fn gender(&self) -> Option<&i32> {
    self.gender.as_ref()
  }

  pub fn reset_gender(&mut self) {
    self.gender = None;
  }

  pub fn set_age_min(&mut self, age_min: i32) {
    self.age_min = Some(age_min);
  }

  pub fn with_age_min(mut self, age_min: i32) -> SecondMarketRequest {
    self.age_min = Some(age_min);
    self
  }

  pub fn age_min(&self) -> Option<&i32> {
    self.age_min.as_ref()
  }

  pub fn reset_age_min(&mut self) {
    self.age_min = None;
  }

  pub fn set_age_max(&mut self, age_max: i32) {
    self.age_max = Some(age_max);
  }

  pub fn with_age_max(mut self, age_max: i32) -> SecondMarketRequest {
    self.age_max = Some(age_max);
    self
  }

  pub fn age_max(&self) -> Option<&i32> {
    self.age_max.as_ref()
  }

  pub fn reset_age_max(&mut self) {
    self.age_max = None;
  }

  pub fn set_income_verification_status(&mut self, income_verification_status: i32) {
    self.income_verification_status = Some(income_verification_status);
  }

  pub fn with_income_verification_status(mut self, income_verification_status: i32) -> SecondMarketRequest {
    self.income_verification_status = Some(income_verification_status);
    self
  }

  pub fn income_verification_status(&self) -> Option<&i32> {
    self.income_verification_status.as_ref()
  }

  pub fn reset_income_verification_status(&mut self) {
    self.income_verification_status = None;
  }

  pub fn set_show_my_items(&mut self, show_my_items: bool) {
    self.show_my_items = Some(show_my_items);
  }

  pub fn with_show_my_items(mut self, show_my_items: bool) -> SecondMarketRequest {
    self.show_my_items = Some(show_my_items);
    self
  }

  pub fn show_my_items(&self) -> Option<&bool> {
    self.show_my_items.as_ref()
  }

  pub fn reset_show_my_items(&mut self) {
    self.show_my_items = None;
  }

  pub fn set_auction_id(&mut self, auction_id: String) {
    self.auction_id = Some(auction_id);
  }

  pub fn with_auction_id(mut self, auction_id: String) -> SecondMarketRequest {
    self.auction_id = Some(auction_id);
    self
  }

  pub fn auction_id(&self) -> Option<&String> {
    self.auction_id.as_ref()
  }

  pub fn reset_auction_id(&mut self) {
    self.auction_id = None;
  }

  pub fn set_listed_on_date_from(&mut self, listed_on_date_from: String) {
    self.listed_on_date_from = Some(listed_on_date_from);
  }

  pub fn with_listed_on_date_from(mut self, listed_on_date_from: String) -> SecondMarketRequest {
    self.listed_on_date_from = Some(listed_on_date_from);
    self
  }

  pub fn listed_on_date_from(&self) -> Option<&String> {
    self.listed_on_date_from.as_ref()
  }

  pub fn reset_listed_on_date_from(&mut self) {
    self.listed_on_date_from = None;
  }

  pub fn set_listed_on_date_to(&mut self, listed_on_date_to: String) {
    self.listed_on_date_to = Some(listed_on_date_to);
  }

  pub fn with_listed_on_date_to(mut self, listed_on_date_to: String) -> SecondMarketRequest {
    self.listed_on_date_to = Some(listed_on_date_to);
    self
  }

  pub fn listed_on_date_to(&self) -> Option<&String> {
    self.listed_on_date_to.as_ref()
  }

  pub fn reset_listed_on_date_to(&mut self) {
    self.listed_on_date_to = None;
  }

  pub fn set_debt_occured_on_from(&mut self, debt_occured_on_from: String) {
    self.debt_occured_on_from = Some(debt_occured_on_from);
  }

  pub fn with_debt_occured_on_from(mut self, debt_occured_on_from: String) -> SecondMarketRequest {
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

  pub fn with_debt_occured_on_to(mut self, debt_occured_on_to: String) -> SecondMarketRequest {
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

  pub fn with_debt_occured_on_for_secondary_from(mut self, debt_occured_on_for_secondary_from: String) -> SecondMarketRequest {
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

  pub fn with_debt_occured_on_for_secondary_to(mut self, debt_occured_on_for_secondary_to: String) -> SecondMarketRequest {
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

  pub fn with_defaulted_date_from(mut self, defaulted_date_from: String) -> SecondMarketRequest {
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

  pub fn with_defaulted_date_to(mut self, defaulted_date_to: String) -> SecondMarketRequest {
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

  pub fn with_rescheduled_from(mut self, rescheduled_from: String) -> SecondMarketRequest {
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

  pub fn with_rescheduled_to(mut self, rescheduled_to: String) -> SecondMarketRequest {
    self.rescheduled_to = Some(rescheduled_to);
    self
  }

  pub fn rescheduled_to(&self) -> Option<&String> {
    self.rescheduled_to.as_ref()
  }

  pub fn reset_rescheduled_to(&mut self) {
    self.rescheduled_to = None;
  }

  pub fn set_last_payment_date_from(&mut self, last_payment_date_from: String) {
    self.last_payment_date_from = Some(last_payment_date_from);
  }

  pub fn with_last_payment_date_from(mut self, last_payment_date_from: String) -> SecondMarketRequest {
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

  pub fn with_last_payment_date_to(mut self, last_payment_date_to: String) -> SecondMarketRequest {
    self.last_payment_date_to = Some(last_payment_date_to);
    self
  }

  pub fn last_payment_date_to(&self) -> Option<&String> {
    self.last_payment_date_to.as_ref()
  }

  pub fn reset_last_payment_date_to(&mut self) {
    self.last_payment_date_to = None;
  }

  pub fn set_next_payment_date_from(&mut self, next_payment_date_from: String) {
    self.next_payment_date_from = Some(next_payment_date_from);
  }

  pub fn with_next_payment_date_from(mut self, next_payment_date_from: String) -> SecondMarketRequest {
    self.next_payment_date_from = Some(next_payment_date_from);
    self
  }

  pub fn next_payment_date_from(&self) -> Option<&String> {
    self.next_payment_date_from.as_ref()
  }

  pub fn reset_next_payment_date_from(&mut self) {
    self.next_payment_date_from = None;
  }

  pub fn set_next_payment_date_to(&mut self, next_payment_date_to: String) {
    self.next_payment_date_to = Some(next_payment_date_to);
  }

  pub fn with_next_payment_date_to(mut self, next_payment_date_to: String) -> SecondMarketRequest {
    self.next_payment_date_to = Some(next_payment_date_to);
    self
  }

  pub fn next_payment_date_to(&self) -> Option<&String> {
    self.next_payment_date_to.as_ref()
  }

  pub fn reset_next_payment_date_to(&mut self) {
    self.next_payment_date_to = None;
  }

  pub fn set_desired_discount_rate_min(&mut self, desired_discount_rate_min: f64) {
    self.desired_discount_rate_min = Some(desired_discount_rate_min);
  }

  pub fn with_desired_discount_rate_min(mut self, desired_discount_rate_min: f64) -> SecondMarketRequest {
    self.desired_discount_rate_min = Some(desired_discount_rate_min);
    self
  }

  pub fn desired_discount_rate_min(&self) -> Option<&f64> {
    self.desired_discount_rate_min.as_ref()
  }

  pub fn reset_desired_discount_rate_min(&mut self) {
    self.desired_discount_rate_min = None;
  }

  pub fn set_desired_discount_rate_max(&mut self, desired_discount_rate_max: f64) {
    self.desired_discount_rate_max = Some(desired_discount_rate_max);
  }

  pub fn with_desired_discount_rate_max(mut self, desired_discount_rate_max: f64) -> SecondMarketRequest {
    self.desired_discount_rate_max = Some(desired_discount_rate_max);
    self
  }

  pub fn desired_discount_rate_max(&self) -> Option<&f64> {
    self.desired_discount_rate_max.as_ref()
  }

  pub fn reset_desired_discount_rate_max(&mut self) {
    self.desired_discount_rate_max = None;
  }

  pub fn set_xirr_min(&mut self, xirr_min: f64) {
    self.xirr_min = Some(xirr_min);
  }

  pub fn with_xirr_min(mut self, xirr_min: f64) -> SecondMarketRequest {
    self.xirr_min = Some(xirr_min);
    self
  }

  pub fn xirr_min(&self) -> Option<&f64> {
    self.xirr_min.as_ref()
  }

  pub fn reset_xirr_min(&mut self) {
    self.xirr_min = None;
  }

  pub fn set_xirr_max(&mut self, xirr_max: f64) {
    self.xirr_max = Some(xirr_max);
  }

  pub fn with_xirr_max(mut self, xirr_max: f64) -> SecondMarketRequest {
    self.xirr_max = Some(xirr_max);
    self
  }

  pub fn xirr_max(&self) -> Option<&f64> {
    self.xirr_max.as_ref()
  }

  pub fn reset_xirr_max(&mut self) {
    self.xirr_max = None;
  }

  pub fn set_page_size(&mut self, page_size: i32) {
    self.page_size = Some(page_size);
  }

  pub fn with_page_size(mut self, page_size: i32) -> SecondMarketRequest {
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

  pub fn with_page_nr(mut self, page_nr: i32) -> SecondMarketRequest {
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



