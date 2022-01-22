
/// BidSummary : Bid's information

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct BidSummary {
  /// Bid unique identifier
  #[serde(rename = "Id")]
  id: Option<String>,
  /// Id of auction to bid into
  #[serde(rename = "AuctionId")]
  auction_id: Option<String>,
  /// Amount that was requested to bid
  #[serde(rename = "RequestedBidAmount")]
  requested_bid_amount: Option<f64>,
  /// Amount that is bidded
  #[serde(rename = "ActualBidAmount")]
  actual_bid_amount: Option<f64>,
  /// Minimum amount that was specified for auction
  #[serde(rename = "RequestedBidMinimumLimit")]
  requested_bid_minimum_limit: Option<f64>,
  /// When bid was requested
  #[serde(rename = "BidRequestedDate")]
  bid_requested_date: Option<String>,
  /// When bid was processed
  #[serde(rename = "BidProcessedDate")]
  bid_processed_date: Option<String>,
  /// Is request currently processed
  #[serde(rename = "IsRequestBeingProcessed")]
  is_request_being_processed: Option<bool>,
  /// Status of bid  <para>0 Pending</para><para>1 Open</para><para>2 Successful</para><para>3 Failed</para><para>4 Cancelled</para><para>5 Accepted</para>
  #[serde(rename = "StatusCode")]
  status_code: Option<i32>,
  /// Why bid failed
  #[serde(rename = "FailureReason")]
  failure_reason: Option<i32>
}

impl BidSummary {
  /// Bid's information
  pub fn new() -> BidSummary {
    BidSummary {
      id: None,
      auction_id: None,
      requested_bid_amount: None,
      actual_bid_amount: None,
      requested_bid_minimum_limit: None,
      bid_requested_date: None,
      bid_processed_date: None,
      is_request_being_processed: None,
      status_code: None,
      failure_reason: None
    }
  }

  pub fn set_id(&mut self, id: String) {
    self.id = Some(id);
  }

  pub fn with_id(mut self, id: String) -> BidSummary {
    self.id = Some(id);
    self
  }

  pub fn id(&self) -> Option<&String> {
    self.id.as_ref()
  }

  pub fn reset_id(&mut self) {
    self.id = None;
  }

  pub fn set_auction_id(&mut self, auction_id: String) {
    self.auction_id = Some(auction_id);
  }

  pub fn with_auction_id(mut self, auction_id: String) -> BidSummary {
    self.auction_id = Some(auction_id);
    self
  }

  pub fn auction_id(&self) -> Option<&String> {
    self.auction_id.as_ref()
  }

  pub fn reset_auction_id(&mut self) {
    self.auction_id = None;
  }

  pub fn set_requested_bid_amount(&mut self, requested_bid_amount: f64) {
    self.requested_bid_amount = Some(requested_bid_amount);
  }

  pub fn with_requested_bid_amount(mut self, requested_bid_amount: f64) -> BidSummary {
    self.requested_bid_amount = Some(requested_bid_amount);
    self
  }

  pub fn requested_bid_amount(&self) -> Option<&f64> {
    self.requested_bid_amount.as_ref()
  }

  pub fn reset_requested_bid_amount(&mut self) {
    self.requested_bid_amount = None;
  }

  pub fn set_actual_bid_amount(&mut self, actual_bid_amount: f64) {
    self.actual_bid_amount = Some(actual_bid_amount);
  }

  pub fn with_actual_bid_amount(mut self, actual_bid_amount: f64) -> BidSummary {
    self.actual_bid_amount = Some(actual_bid_amount);
    self
  }

  pub fn actual_bid_amount(&self) -> Option<&f64> {
    self.actual_bid_amount.as_ref()
  }

  pub fn reset_actual_bid_amount(&mut self) {
    self.actual_bid_amount = None;
  }

  pub fn set_requested_bid_minimum_limit(&mut self, requested_bid_minimum_limit: f64) {
    self.requested_bid_minimum_limit = Some(requested_bid_minimum_limit);
  }

  pub fn with_requested_bid_minimum_limit(mut self, requested_bid_minimum_limit: f64) -> BidSummary {
    self.requested_bid_minimum_limit = Some(requested_bid_minimum_limit);
    self
  }

  pub fn requested_bid_minimum_limit(&self) -> Option<&f64> {
    self.requested_bid_minimum_limit.as_ref()
  }

  pub fn reset_requested_bid_minimum_limit(&mut self) {
    self.requested_bid_minimum_limit = None;
  }

  pub fn set_bid_requested_date(&mut self, bid_requested_date: String) {
    self.bid_requested_date = Some(bid_requested_date);
  }

  pub fn with_bid_requested_date(mut self, bid_requested_date: String) -> BidSummary {
    self.bid_requested_date = Some(bid_requested_date);
    self
  }

  pub fn bid_requested_date(&self) -> Option<&String> {
    self.bid_requested_date.as_ref()
  }

  pub fn reset_bid_requested_date(&mut self) {
    self.bid_requested_date = None;
  }

  pub fn set_bid_processed_date(&mut self, bid_processed_date: String) {
    self.bid_processed_date = Some(bid_processed_date);
  }

  pub fn with_bid_processed_date(mut self, bid_processed_date: String) -> BidSummary {
    self.bid_processed_date = Some(bid_processed_date);
    self
  }

  pub fn bid_processed_date(&self) -> Option<&String> {
    self.bid_processed_date.as_ref()
  }

  pub fn reset_bid_processed_date(&mut self) {
    self.bid_processed_date = None;
  }

  pub fn set_is_request_being_processed(&mut self, is_request_being_processed: bool) {
    self.is_request_being_processed = Some(is_request_being_processed);
  }

  pub fn with_is_request_being_processed(mut self, is_request_being_processed: bool) -> BidSummary {
    self.is_request_being_processed = Some(is_request_being_processed);
    self
  }

  pub fn is_request_being_processed(&self) -> Option<&bool> {
    self.is_request_being_processed.as_ref()
  }

  pub fn reset_is_request_being_processed(&mut self) {
    self.is_request_being_processed = None;
  }

  pub fn set_status_code(&mut self, status_code: i32) {
    self.status_code = Some(status_code);
  }

  pub fn with_status_code(mut self, status_code: i32) -> BidSummary {
    self.status_code = Some(status_code);
    self
  }

  pub fn status_code(&self) -> Option<&i32> {
    self.status_code.as_ref()
  }

  pub fn reset_status_code(&mut self) {
    self.status_code = None;
  }

  pub fn set_failure_reason(&mut self, failure_reason: i32) {
    self.failure_reason = Some(failure_reason);
  }

  pub fn with_failure_reason(mut self, failure_reason: i32) -> BidSummary {
    self.failure_reason = Some(failure_reason);
    self
  }

  pub fn failure_reason(&self) -> Option<&i32> {
    self.failure_reason.as_ref()
  }

  pub fn reset_failure_reason(&mut self) {
    self.failure_reason = None;
  }

}



