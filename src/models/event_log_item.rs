/// EventLogItem : EventLog list item's information
#[derive(Debug, Serialize, Deserialize)]
pub struct EventLogItem {
    /// Event date
    #[serde(rename = "EventDate")]
    pub event_date: Option<String>,
    /// Event type
    #[serde(rename = "EventType")]
    pub event_type: Option<i32>,
    /// IP address
    #[serde(rename = "IpAddress")]
    pub ip_address: Option<String>,
    /// Request data JSON format
    #[serde(rename = "Data")]
    pub data: Option<String>,
}
