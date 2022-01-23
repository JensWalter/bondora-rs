
/// AuctionRequest : Request object for filtering auctions

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AuctionRequest {
  /// Two letter iso code for country of origin: EE, ES, FI
  #[serde(rename = "Countries")]
  pub countries: Option<Vec<String>>,
  /// Bondora's rating: AA, A, B, C, D, E, F, HR
  #[serde(rename = "Ratings")]
  pub ratings: Option<Vec<String>>,
  /// Borrower's gender: Male 0, Female 1, Unknown 2
  #[serde(rename = "Gender")]
  pub gender: Option<i32>,
  /// Minimal loan amount
  #[serde(rename = "SumMin")]
  pub sum_min: Option<i32>,
  /// Maximum loan amount
  #[serde(rename = "SumMax")]
  pub sum_max: Option<i32>,
  /// Loan length: 3, 9, 12, 18, 24, 36, 48, 60 months
  #[serde(rename = "Terms")]
  pub terms: Option<Vec<i32>>,
  /// Minimal age
  #[serde(rename = "AgeMin")]
  pub age_min: Option<i32>,
  /// Maximum age
  #[serde(rename = "AgeMax")]
  pub age_max: Option<i32>,
  /// Loan number
  #[serde(rename = "LoanNumber")]
  pub loan_number: Option<i32>,
  /// Username
  #[serde(rename = "UserName")]
  pub user_name: Option<String>,
  /// Loan application started date from
  #[serde(rename = "ApplicationDateFrom")]
  pub application_date_from: Option<String>,
  /// Loan application started date to
  #[serde(rename = "ApplicationDateTo")]
  pub application_date_to: Option<String>,
  /// Minimum credit score
  #[serde(rename = "CreditScoreMin")]
  pub credit_score_min: Option<i32>,
  /// Maximum credit score
  #[serde(rename = "CreditScoreMax")]
  pub credit_score_max: Option<i32>,
  /// Credit score for EE loans
  #[serde(rename = "CreditScoresEeMini")]
  pub credit_scores_ee_mini: Option<Vec<String>>,
  /// Minimum interest
  #[serde(rename = "InterestMin")]
  pub interest_min: Option<f64>,
  /// Maximum interest
  #[serde(rename = "InterestMax")]
  pub interest_max: Option<f64>,
  /// Minimal total income
  #[serde(rename = "IncomeTotalMin")]
  pub income_total_min: Option<f64>,
  /// Maximum total income
  #[serde(rename = "IncomeTotalMax")]
  pub income_total_max: Option<f64>,
  /// Model version
  #[serde(rename = "ModelVersion")]
  pub model_version: Option<i32>,
  /// Minimal expected loss
  #[serde(rename = "ExpectedLossMin")]
  pub expected_loss_min: Option<f64>,
  /// Maximum expected loss
  #[serde(rename = "ExpectedLossMax")]
  pub expected_loss_max: Option<f64>,
  /// Date when auction was published from
  #[serde(rename = "ListedOnUTCFrom")]
  pub listed_on_utc_from: Option<String>,
  /// Date when auction was published to
  #[serde(rename = "ListedOnUTCTo")]
  pub listed_on_utc_to: Option<String>,
  /// Max items in result, up to 20000
  #[serde(rename = "PageSize")]
  pub page_size: Option<i32>,
  /// Result page nr
  #[serde(rename = "PageNr")]
  pub page_nr: Option<i32>
}
