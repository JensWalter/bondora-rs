/// EventLogRequest : Request object for filtering event log
#[derive(Debug, Serialize, Deserialize)]
pub struct EventLogRequest {
    /// Start datetime
    #[serde(rename = "EventDateFrom")]
    pub event_date_from: Option<String>,
    /// end datetime
    #[serde(rename = "EventDateTo")]
    pub event_date_to: Option<String>,
    /// Event type
    #[serde(rename = "EventType")]
    pub event_type: Option<i32>,
    /// IP address
    #[serde(rename = "IpAddress")]
    pub ip_address: Option<String>,
    /// Max items in result, up to 20000
    #[serde(rename = "PageSize")]
    pub page_size: Option<i32>,
    /// Result page nr
    #[serde(rename = "PageNr")]
    pub page_nr: Option<i32>,
}
