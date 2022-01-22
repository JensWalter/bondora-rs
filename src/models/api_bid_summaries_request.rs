/// ApiBidSummariesRequest : Request object for filtering api bids

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiBidSummariesRequest {
  /// Bid status code
  #[serde(rename = "BidStatusCode")]
  bid_status_code: Option<i32>,
  /// Start date
  #[serde(rename = "StartDate")]
  start_date: Option<String>,
  /// End date
  #[serde(rename = "EndDate")]
  end_date: Option<String>,
  /// Max items in result, up to 20000
  #[serde(rename = "PageSize")]
  page_size: Option<i32>,
  /// Result page nr
  #[serde(rename = "PageNr")]
  page_nr: Option<i32>
}

impl ApiBidSummariesRequest {
  /// Request object for filtering api bids
  pub fn new() -> ApiBidSummariesRequest {
    ApiBidSummariesRequest {
      bid_status_code: None,
      start_date: None,
      end_date: None,
      page_size: None,
      page_nr: None
    }
  }

  pub fn set_bid_status_code(&mut self, bid_status_code: i32) {
    self.bid_status_code = Some(bid_status_code);
  }

  pub fn with_bid_status_code(mut self, bid_status_code: i32) -> ApiBidSummariesRequest {
    self.bid_status_code = Some(bid_status_code);
    self
  }

  pub fn bid_status_code(&self) -> Option<&i32> {
    self.bid_status_code.as_ref()
  }

  pub fn reset_bid_status_code(&mut self) {
    self.bid_status_code = None;
  }

  pub fn set_start_date(&mut self, start_date: String) {
    self.start_date = Some(start_date);
  }

  pub fn with_start_date(mut self, start_date: String) -> ApiBidSummariesRequest {
    self.start_date = Some(start_date);
    self
  }

  pub fn start_date(&self) -> Option<&String> {
    self.start_date.as_ref()
  }

  pub fn reset_start_date(&mut self) {
    self.start_date = None;
  }

  pub fn set_end_date(&mut self, end_date: String) {
    self.end_date = Some(end_date);
  }

  pub fn with_end_date(mut self, end_date: String) -> ApiBidSummariesRequest {
    self.end_date = Some(end_date);
    self
  }

  pub fn end_date(&self) -> Option<&String> {
    self.end_date.as_ref()
  }

  pub fn reset_end_date(&mut self) {
    self.end_date = None;
  }

  pub fn set_page_size(&mut self, page_size: i32) {
    self.page_size = Some(page_size);
  }

  pub fn with_page_size(mut self, page_size: i32) -> ApiBidSummariesRequest {
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

  pub fn with_page_nr(mut self, page_nr: i32) -> ApiBidSummariesRequest {
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



