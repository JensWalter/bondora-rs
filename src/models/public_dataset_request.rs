
/// PublicDatasetRequest : Request parameters for getting loan public data

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct PublicDatasetRequest {
  /// Specific loans to search
  #[serde(rename = "LoanIds")]
  loan_ids: Option<Vec<String>>,
  /// Two letter iso code for country of origin: EE, ES, FI
  #[serde(rename = "Countries")]
  countries: Option<Vec<String>>,
  /// Bondora's rating: AA, A, B, C, D, E, F, HR
  #[serde(rename = "Ratings")]
  ratings: Option<Vec<String>>,
  /// Loan start date from
  #[serde(rename = "LoanDateFrom")]
  loan_date_from: Option<String>,
  /// Loan start date to
  #[serde(rename = "LoanDateTo")]
  loan_date_to: Option<String>,
  /// Max items in result, up to 10000
  #[serde(rename = "PageSize")]
  page_size: Option<i32>,
  /// Result page nr
  #[serde(rename = "PageNr")]
  page_nr: Option<i32>
}

impl PublicDatasetRequest {
  /// Request parameters for getting loan public data
  pub fn new() -> PublicDatasetRequest {
    PublicDatasetRequest {
      loan_ids: None,
      countries: None,
      ratings: None,
      loan_date_from: None,
      loan_date_to: None,
      page_size: None,
      page_nr: None
    }
  }

  pub fn set_loan_ids(&mut self, loan_ids: Vec<String>) {
    self.loan_ids = Some(loan_ids);
  }

  pub fn with_loan_ids(mut self, loan_ids: Vec<String>) -> PublicDatasetRequest {
    self.loan_ids = Some(loan_ids);
    self
  }

  pub fn loan_ids(&self) -> Option<&Vec<String>> {
    self.loan_ids.as_ref()
  }

  pub fn reset_loan_ids(&mut self) {
    self.loan_ids = None;
  }

  pub fn set_countries(&mut self, countries: Vec<String>) {
    self.countries = Some(countries);
  }

  pub fn with_countries(mut self, countries: Vec<String>) -> PublicDatasetRequest {
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

  pub fn with_ratings(mut self, ratings: Vec<String>) -> PublicDatasetRequest {
    self.ratings = Some(ratings);
    self
  }

  pub fn ratings(&self) -> Option<&Vec<String>> {
    self.ratings.as_ref()
  }

  pub fn reset_ratings(&mut self) {
    self.ratings = None;
  }

  pub fn set_loan_date_from(&mut self, loan_date_from: String) {
    self.loan_date_from = Some(loan_date_from);
  }

  pub fn with_loan_date_from(mut self, loan_date_from: String) -> PublicDatasetRequest {
    self.loan_date_from = Some(loan_date_from);
    self
  }

  pub fn loan_date_from(&self) -> Option<&String> {
    self.loan_date_from.as_ref()
  }

  pub fn reset_loan_date_from(&mut self) {
    self.loan_date_from = None;
  }

  pub fn set_loan_date_to(&mut self, loan_date_to: String) {
    self.loan_date_to = Some(loan_date_to);
  }

  pub fn with_loan_date_to(mut self, loan_date_to: String) -> PublicDatasetRequest {
    self.loan_date_to = Some(loan_date_to);
    self
  }

  pub fn loan_date_to(&self) -> Option<&String> {
    self.loan_date_to.as_ref()
  }

  pub fn reset_loan_date_to(&mut self) {
    self.loan_date_to = None;
  }

  pub fn set_page_size(&mut self, page_size: i32) {
    self.page_size = Some(page_size);
  }

  pub fn with_page_size(mut self, page_size: i32) -> PublicDatasetRequest {
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

  pub fn with_page_nr(mut self, page_nr: i32) -> PublicDatasetRequest {
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



