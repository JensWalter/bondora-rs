
/// AuctionRequest : Request object for filtering auctions

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct AuctionRequest {
  /// Two letter iso code for country of origin: EE, ES, FI
  #[serde(rename = "Countries")]
  countries: Option<Vec<String>>,
  /// Bondora's rating: AA, A, B, C, D, E, F, HR
  #[serde(rename = "Ratings")]
  ratings: Option<Vec<String>>,
  /// Borrower's gender: Male 0, Female 1, Unknown 2
  #[serde(rename = "Gender")]
  gender: Option<i32>,
  /// Minimal loan amount
  #[serde(rename = "SumMin")]
  sum_min: Option<i32>,
  /// Maximum loan amount
  #[serde(rename = "SumMax")]
  sum_max: Option<i32>,
  /// Loan length: 3, 9, 12, 18, 24, 36, 48, 60 months
  #[serde(rename = "Terms")]
  terms: Option<Vec<i32>>,
  /// Minimal age
  #[serde(rename = "AgeMin")]
  age_min: Option<i32>,
  /// Maximum age
  #[serde(rename = "AgeMax")]
  age_max: Option<i32>,
  /// Loan number
  #[serde(rename = "LoanNumber")]
  loan_number: Option<i32>,
  /// Username
  #[serde(rename = "UserName")]
  user_name: Option<String>,
  /// Loan application started date from
  #[serde(rename = "ApplicationDateFrom")]
  application_date_from: Option<String>,
  /// Loan application started date to
  #[serde(rename = "ApplicationDateTo")]
  application_date_to: Option<String>,
  /// Minimum credit score
  #[serde(rename = "CreditScoreMin")]
  credit_score_min: Option<i32>,
  /// Maximum credit score
  #[serde(rename = "CreditScoreMax")]
  credit_score_max: Option<i32>,
  /// Credit score for EE loans
  #[serde(rename = "CreditScoresEeMini")]
  credit_scores_ee_mini: Option<Vec<String>>,
  /// Minimum interest
  #[serde(rename = "InterestMin")]
  interest_min: Option<f64>,
  /// Maximum interest
  #[serde(rename = "InterestMax")]
  interest_max: Option<f64>,
  /// Minimal total income
  #[serde(rename = "IncomeTotalMin")]
  income_total_min: Option<f64>,
  /// Maximum total income
  #[serde(rename = "IncomeTotalMax")]
  income_total_max: Option<f64>,
  /// Model version
  #[serde(rename = "ModelVersion")]
  model_version: Option<i32>,
  /// Minimal expected loss
  #[serde(rename = "ExpectedLossMin")]
  expected_loss_min: Option<f64>,
  /// Maximum expected loss
  #[serde(rename = "ExpectedLossMax")]
  expected_loss_max: Option<f64>,
  /// Date when auction was published from
  #[serde(rename = "ListedOnUTCFrom")]
  listed_on_utc_from: Option<String>,
  /// Date when auction was published to
  #[serde(rename = "ListedOnUTCTo")]
  listed_on_utc_to: Option<String>,
  /// Max items in result, up to 20000
  #[serde(rename = "PageSize")]
  page_size: Option<i32>,
  /// Result page nr
  #[serde(rename = "PageNr")]
  page_nr: Option<i32>
}

impl AuctionRequest {
  /// Request object for filtering auctions
  pub fn new() -> AuctionRequest {
    AuctionRequest {
      countries: None,
      ratings: None,
      gender: None,
      sum_min: None,
      sum_max: None,
      terms: None,
      age_min: None,
      age_max: None,
      loan_number: None,
      user_name: None,
      application_date_from: None,
      application_date_to: None,
      credit_score_min: None,
      credit_score_max: None,
      credit_scores_ee_mini: None,
      interest_min: None,
      interest_max: None,
      income_total_min: None,
      income_total_max: None,
      model_version: None,
      expected_loss_min: None,
      expected_loss_max: None,
      listed_on_utc_from: None,
      listed_on_utc_to: None,
      page_size: None,
      page_nr: None
    }
  }

  pub fn set_countries(&mut self, countries: Vec<String>) {
    self.countries = Some(countries);
  }

  pub fn with_countries(mut self, countries: Vec<String>) -> AuctionRequest {
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

  pub fn with_ratings(mut self, ratings: Vec<String>) -> AuctionRequest {
    self.ratings = Some(ratings);
    self
  }

  pub fn ratings(&self) -> Option<&Vec<String>> {
    self.ratings.as_ref()
  }

  pub fn reset_ratings(&mut self) {
    self.ratings = None;
  }

  pub fn set_gender(&mut self, gender: i32) {
    self.gender = Some(gender);
  }

  pub fn with_gender(mut self, gender: i32) -> AuctionRequest {
    self.gender = Some(gender);
    self
  }

  pub fn gender(&self) -> Option<&i32> {
    self.gender.as_ref()
  }

  pub fn reset_gender(&mut self) {
    self.gender = None;
  }

  pub fn set_sum_min(&mut self, sum_min: i32) {
    self.sum_min = Some(sum_min);
  }

  pub fn with_sum_min(mut self, sum_min: i32) -> AuctionRequest {
    self.sum_min = Some(sum_min);
    self
  }

  pub fn sum_min(&self) -> Option<&i32> {
    self.sum_min.as_ref()
  }

  pub fn reset_sum_min(&mut self) {
    self.sum_min = None;
  }

  pub fn set_sum_max(&mut self, sum_max: i32) {
    self.sum_max = Some(sum_max);
  }

  pub fn with_sum_max(mut self, sum_max: i32) -> AuctionRequest {
    self.sum_max = Some(sum_max);
    self
  }

  pub fn sum_max(&self) -> Option<&i32> {
    self.sum_max.as_ref()
  }

  pub fn reset_sum_max(&mut self) {
    self.sum_max = None;
  }

  pub fn set_terms(&mut self, terms: Vec<i32>) {
    self.terms = Some(terms);
  }

  pub fn with_terms(mut self, terms: Vec<i32>) -> AuctionRequest {
    self.terms = Some(terms);
    self
  }

  pub fn terms(&self) -> Option<&Vec<i32>> {
    self.terms.as_ref()
  }

  pub fn reset_terms(&mut self) {
    self.terms = None;
  }

  pub fn set_age_min(&mut self, age_min: i32) {
    self.age_min = Some(age_min);
  }

  pub fn with_age_min(mut self, age_min: i32) -> AuctionRequest {
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

  pub fn with_age_max(mut self, age_max: i32) -> AuctionRequest {
    self.age_max = Some(age_max);
    self
  }

  pub fn age_max(&self) -> Option<&i32> {
    self.age_max.as_ref()
  }

  pub fn reset_age_max(&mut self) {
    self.age_max = None;
  }

  pub fn set_loan_number(&mut self, loan_number: i32) {
    self.loan_number = Some(loan_number);
  }

  pub fn with_loan_number(mut self, loan_number: i32) -> AuctionRequest {
    self.loan_number = Some(loan_number);
    self
  }

  pub fn loan_number(&self) -> Option<&i32> {
    self.loan_number.as_ref()
  }

  pub fn reset_loan_number(&mut self) {
    self.loan_number = None;
  }

  pub fn set_user_name(&mut self, user_name: String) {
    self.user_name = Some(user_name);
  }

  pub fn with_user_name(mut self, user_name: String) -> AuctionRequest {
    self.user_name = Some(user_name);
    self
  }

  pub fn user_name(&self) -> Option<&String> {
    self.user_name.as_ref()
  }

  pub fn reset_user_name(&mut self) {
    self.user_name = None;
  }

  pub fn set_application_date_from(&mut self, application_date_from: String) {
    self.application_date_from = Some(application_date_from);
  }

  pub fn with_application_date_from(mut self, application_date_from: String) -> AuctionRequest {
    self.application_date_from = Some(application_date_from);
    self
  }

  pub fn application_date_from(&self) -> Option<&String> {
    self.application_date_from.as_ref()
  }

  pub fn reset_application_date_from(&mut self) {
    self.application_date_from = None;
  }

  pub fn set_application_date_to(&mut self, application_date_to: String) {
    self.application_date_to = Some(application_date_to);
  }

  pub fn with_application_date_to(mut self, application_date_to: String) -> AuctionRequest {
    self.application_date_to = Some(application_date_to);
    self
  }

  pub fn application_date_to(&self) -> Option<&String> {
    self.application_date_to.as_ref()
  }

  pub fn reset_application_date_to(&mut self) {
    self.application_date_to = None;
  }

  pub fn set_credit_score_min(&mut self, credit_score_min: i32) {
    self.credit_score_min = Some(credit_score_min);
  }

  pub fn with_credit_score_min(mut self, credit_score_min: i32) -> AuctionRequest {
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

  pub fn with_credit_score_max(mut self, credit_score_max: i32) -> AuctionRequest {
    self.credit_score_max = Some(credit_score_max);
    self
  }

  pub fn credit_score_max(&self) -> Option<&i32> {
    self.credit_score_max.as_ref()
  }

  pub fn reset_credit_score_max(&mut self) {
    self.credit_score_max = None;
  }

  pub fn set_credit_scores_ee_mini(&mut self, credit_scores_ee_mini: Vec<String>) {
    self.credit_scores_ee_mini = Some(credit_scores_ee_mini);
  }

  pub fn with_credit_scores_ee_mini(mut self, credit_scores_ee_mini: Vec<String>) -> AuctionRequest {
    self.credit_scores_ee_mini = Some(credit_scores_ee_mini);
    self
  }

  pub fn credit_scores_ee_mini(&self) -> Option<&Vec<String>> {
    self.credit_scores_ee_mini.as_ref()
  }

  pub fn reset_credit_scores_ee_mini(&mut self) {
    self.credit_scores_ee_mini = None;
  }

  pub fn set_interest_min(&mut self, interest_min: f64) {
    self.interest_min = Some(interest_min);
  }

  pub fn with_interest_min(mut self, interest_min: f64) -> AuctionRequest {
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

  pub fn with_interest_max(mut self, interest_max: f64) -> AuctionRequest {
    self.interest_max = Some(interest_max);
    self
  }

  pub fn interest_max(&self) -> Option<&f64> {
    self.interest_max.as_ref()
  }

  pub fn reset_interest_max(&mut self) {
    self.interest_max = None;
  }

  pub fn set_income_total_min(&mut self, income_total_min: f64) {
    self.income_total_min = Some(income_total_min);
  }

  pub fn with_income_total_min(mut self, income_total_min: f64) -> AuctionRequest {
    self.income_total_min = Some(income_total_min);
    self
  }

  pub fn income_total_min(&self) -> Option<&f64> {
    self.income_total_min.as_ref()
  }

  pub fn reset_income_total_min(&mut self) {
    self.income_total_min = None;
  }

  pub fn set_income_total_max(&mut self, income_total_max: f64) {
    self.income_total_max = Some(income_total_max);
  }

  pub fn with_income_total_max(mut self, income_total_max: f64) -> AuctionRequest {
    self.income_total_max = Some(income_total_max);
    self
  }

  pub fn income_total_max(&self) -> Option<&f64> {
    self.income_total_max.as_ref()
  }

  pub fn reset_income_total_max(&mut self) {
    self.income_total_max = None;
  }

  pub fn set_model_version(&mut self, model_version: i32) {
    self.model_version = Some(model_version);
  }

  pub fn with_model_version(mut self, model_version: i32) -> AuctionRequest {
    self.model_version = Some(model_version);
    self
  }

  pub fn model_version(&self) -> Option<&i32> {
    self.model_version.as_ref()
  }

  pub fn reset_model_version(&mut self) {
    self.model_version = None;
  }

  pub fn set_expected_loss_min(&mut self, expected_loss_min: f64) {
    self.expected_loss_min = Some(expected_loss_min);
  }

  pub fn with_expected_loss_min(mut self, expected_loss_min: f64) -> AuctionRequest {
    self.expected_loss_min = Some(expected_loss_min);
    self
  }

  pub fn expected_loss_min(&self) -> Option<&f64> {
    self.expected_loss_min.as_ref()
  }

  pub fn reset_expected_loss_min(&mut self) {
    self.expected_loss_min = None;
  }

  pub fn set_expected_loss_max(&mut self, expected_loss_max: f64) {
    self.expected_loss_max = Some(expected_loss_max);
  }

  pub fn with_expected_loss_max(mut self, expected_loss_max: f64) -> AuctionRequest {
    self.expected_loss_max = Some(expected_loss_max);
    self
  }

  pub fn expected_loss_max(&self) -> Option<&f64> {
    self.expected_loss_max.as_ref()
  }

  pub fn reset_expected_loss_max(&mut self) {
    self.expected_loss_max = None;
  }

  pub fn set_listed_on_utc_from(&mut self, listed_on_utc_from: String) {
    self.listed_on_utc_from = Some(listed_on_utc_from);
  }

  pub fn with_listed_on_utc_from(mut self, listed_on_utc_from: String) -> AuctionRequest {
    self.listed_on_utc_from = Some(listed_on_utc_from);
    self
  }

  pub fn listed_on_utc_from(&self) -> Option<&String> {
    self.listed_on_utc_from.as_ref()
  }

  pub fn reset_listed_on_utc_from(&mut self) {
    self.listed_on_utc_from = None;
  }

  pub fn set_listed_on_utc_to(&mut self, listed_on_utc_to: String) {
    self.listed_on_utc_to = Some(listed_on_utc_to);
  }

  pub fn with_listed_on_utc_to(mut self, listed_on_utc_to: String) -> AuctionRequest {
    self.listed_on_utc_to = Some(listed_on_utc_to);
    self
  }

  pub fn listed_on_utc_to(&self) -> Option<&String> {
    self.listed_on_utc_to.as_ref()
  }

  pub fn reset_listed_on_utc_to(&mut self) {
    self.listed_on_utc_to = None;
  }

  pub fn set_page_size(&mut self, page_size: i32) {
    self.page_size = Some(page_size);
  }

  pub fn with_page_size(mut self, page_size: i32) -> AuctionRequest {
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

  pub fn with_page_nr(mut self, page_nr: i32) -> AuctionRequest {
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



