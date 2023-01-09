/// SecondMarketSaleRequest : Secondary market sale request
#[derive(Debug, Serialize, Deserialize)]
pub struct SecondMarketSaleRequest {
    /// LoanParts to sell
    #[serde(rename = "Items")]
    pub items: Option<Vec<crate::models::SecondMarketSell>>,
    /// Allows auto cancellation of loans on sale if they receive new repayments
    #[serde(rename = "CancelItemOnPaymentReceived")]
    pub cancel_item_on_payment_received: Option<bool>,
    /// Allows auto cancellation of loans on sale if they are rescheduled
    #[serde(rename = "CancelItemOnReschedule")]
    pub cancel_item_on_reschedule: Option<bool>,
}
