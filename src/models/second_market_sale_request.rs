
/// SecondMarketSaleRequest : Secondary market sale request

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct SecondMarketSaleRequest {
  /// LoanParts to sell
  #[serde(rename = "Items")]
  items: Option<Vec<crate::models::SecondMarketSell>>,
  /// Allows auto cancellation of loans on sale if they receive new repayments
  #[serde(rename = "CancelItemOnPaymentReceived")]
  cancel_item_on_payment_received: Option<bool>,
  /// Allows auto cancellation of loans on sale if they are rescheduled
  #[serde(rename = "CancelItemOnReschedule")]
  cancel_item_on_reschedule: Option<bool>
}

impl SecondMarketSaleRequest {
  /// Secondary market sale request
  pub fn new() -> SecondMarketSaleRequest {
    SecondMarketSaleRequest {
      items: None,
      cancel_item_on_payment_received: None,
      cancel_item_on_reschedule: None
    }
  }

  pub fn set_items(&mut self, items: Vec<crate::models::SecondMarketSell>) {
    self.items = Some(items);
  }

  pub fn with_items(mut self, items: Vec<crate::models::SecondMarketSell>) -> SecondMarketSaleRequest {
    self.items = Some(items);
    self
  }

  pub fn items(&self) -> Option<&Vec<crate::models::SecondMarketSell>> {
    self.items.as_ref()
  }

  pub fn reset_items(&mut self) {
    self.items = None;
  }

  pub fn set_cancel_item_on_payment_received(&mut self, cancel_item_on_payment_received: bool) {
    self.cancel_item_on_payment_received = Some(cancel_item_on_payment_received);
  }

  pub fn with_cancel_item_on_payment_received(mut self, cancel_item_on_payment_received: bool) -> SecondMarketSaleRequest {
    self.cancel_item_on_payment_received = Some(cancel_item_on_payment_received);
    self
  }

  pub fn cancel_item_on_payment_received(&self) -> Option<&bool> {
    self.cancel_item_on_payment_received.as_ref()
  }

  pub fn reset_cancel_item_on_payment_received(&mut self) {
    self.cancel_item_on_payment_received = None;
  }

  pub fn set_cancel_item_on_reschedule(&mut self, cancel_item_on_reschedule: bool) {
    self.cancel_item_on_reschedule = Some(cancel_item_on_reschedule);
  }

  pub fn with_cancel_item_on_reschedule(mut self, cancel_item_on_reschedule: bool) -> SecondMarketSaleRequest {
    self.cancel_item_on_reschedule = Some(cancel_item_on_reschedule);
    self
  }

  pub fn cancel_item_on_reschedule(&self) -> Option<&bool> {
    self.cancel_item_on_reschedule.as_ref()
  }

  pub fn reset_cancel_item_on_reschedule(&mut self) {
    self.cancel_item_on_reschedule = None;
  }

}



